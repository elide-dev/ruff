use anyhow::Result;

use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast as ast;
use ruff_python_ast::ParameterWithDefault;
use ruff_python_codegen::Stylist;
use ruff_python_semantic::analyze::class::{IsMetaclass, is_metaclass};
use ruff_python_semantic::analyze::function_type;
use ruff_python_semantic::{Scope, ScopeKind, SemanticModel};
use ruff_text_size::{Ranged, TextRange};

use crate::checkers::ast::{Checker, DiagnosticGuard};
use crate::registry::Rule;
use crate::renamer::{Renamer, ShadowedKind};
use crate::{Fix, Violation};

/// ## What it does
/// Checks for instance methods that use a name other than `self` for their
/// first argument.
///
/// ## Why is this bad?
/// [PEP 8] recommends the use of `self` as first argument for all instance
/// methods:
///
/// > Always use self for the first argument to instance methods.
/// >
/// > If a function argument’s name clashes with a reserved keyword, it is generally better to
/// > append a single trailing underscore rather than use an abbreviation or spelling corruption.
/// > Thus `class_` is better than `clss`. (Perhaps better is to avoid such clashes by using a synonym.)
///
/// Names can be excluded from this rule using the [`lint.pep8-naming.ignore-names`]
/// or [`lint.pep8-naming.extend-ignore-names`] configuration options. For example,
/// to allow the use of `this` as the first argument to instance methods, set
/// the [`lint.pep8-naming.extend-ignore-names`] option to `["this"]`.
///
/// ## Example
///
/// ```python
/// class Example:
///     def function(cls, data): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Example:
///     def function(self, data): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as renaming a method parameter
/// can change the behavior of the program.
///
/// ## Options
/// - `lint.pep8-naming.classmethod-decorators`
/// - `lint.pep8-naming.staticmethod-decorators`
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-method-arguments
#[derive(ViolationMetadata)]
pub(crate) struct InvalidFirstArgumentNameForMethod {
    argument_name: String,
}

impl Violation for InvalidFirstArgumentNameForMethod {
    const FIX_AVAILABILITY: crate::FixAvailability = crate::FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "First argument of a method should be named `self`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        let Self { argument_name } = self;
        Some(format!("Rename `{argument_name}` to `self`"))
    }
}

/// ## What it does
/// Checks for class methods that use a name other than `cls` for their
/// first argument.
///
/// The method `__new__` is exempted from this
/// check and the corresponding violation is caught by
/// [`bad-staticmethod-argument`][PLW0211].
///
/// ## Why is this bad?
/// [PEP 8] recommends the use of `cls` as the first argument for all class
/// methods:
///
/// > Always use `cls` for the first argument to class methods.
/// >
/// > If a function argument’s name clashes with a reserved keyword, it is generally better to
/// > append a single trailing underscore rather than use an abbreviation or spelling corruption.
/// > Thus `class_` is better than `clss`. (Perhaps better is to avoid such clashes by using a synonym.)
///
/// Names can be excluded from this rule using the [`lint.pep8-naming.ignore-names`]
/// or [`lint.pep8-naming.extend-ignore-names`] configuration options. For example,
/// to allow the use of `klass` as the first argument to class methods, set
/// the [`lint.pep8-naming.extend-ignore-names`] option to `["klass"]`.
///
/// ## Example
///
/// ```python
/// class Example:
///     @classmethod
///     def function(self, data): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Example:
///     @classmethod
///     def function(cls, data): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as renaming a method parameter
/// can change the behavior of the program.
///
/// ## Options
/// - `lint.pep8-naming.classmethod-decorators`
/// - `lint.pep8-naming.staticmethod-decorators`
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-method-arguments
/// [PLW0211]: https://docs.astral.sh/ruff/rules/bad-staticmethod-argument/
#[derive(ViolationMetadata)]
pub(crate) struct InvalidFirstArgumentNameForClassMethod {
    argument_name: String,
    // Whether the method is `__new__`
    is_new: bool,
}

impl Violation for InvalidFirstArgumentNameForClassMethod {
    const FIX_AVAILABILITY: crate::FixAvailability = crate::FixAvailability::Sometimes;

    #[derive_message_formats]
    // The first string below is what shows up in the documentation
    // in the rule table, and it is the more common case.
    #[expect(clippy::if_not_else)]
    fn message(&self) -> String {
        if !self.is_new {
            "First argument of a class method should be named `cls`".to_string()
        } else {
            "First argument of `__new__` method should be named `cls`".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Self { argument_name, .. } = self;
        Some(format!("Rename `{argument_name}` to `cls`"))
    }
}

#[derive(Debug, Copy, Clone)]
enum FunctionType {
    /// The function is an instance method.
    Method,
    /// The function is a class method.
    ClassMethod,
}

impl FunctionType {
    fn diagnostic_kind<'a, 'b>(
        self,
        checker: &'a Checker<'b>,
        argument_name: String,
        range: TextRange,
    ) -> DiagnosticGuard<'a, 'b> {
        match self {
            Self::Method => checker
                .report_diagnostic(InvalidFirstArgumentNameForMethod { argument_name }, range),
            Self::ClassMethod => checker.report_diagnostic(
                InvalidFirstArgumentNameForClassMethod {
                    argument_name,
                    is_new: false,
                },
                range,
            ),
        }
    }

    const fn valid_first_argument_name(self) -> &'static str {
        match self {
            Self::Method => "self",
            Self::ClassMethod => "cls",
        }
    }

    const fn rule(self) -> Rule {
        match self {
            Self::Method => Rule::InvalidFirstArgumentNameForMethod,
            Self::ClassMethod => Rule::InvalidFirstArgumentNameForClassMethod,
        }
    }
}

/// N804, N805
pub(crate) fn invalid_first_argument_name(checker: &Checker, scope: &Scope) {
    let ScopeKind::Function(ast::StmtFunctionDef {
        name,
        parameters,
        decorator_list,
        ..
    }) = &scope.kind
    else {
        panic!("Expected ScopeKind::Function")
    };

    let semantic = checker.semantic();

    let Some(parent_scope) = semantic.first_non_type_parent_scope(scope) else {
        return;
    };

    let ScopeKind::Class(parent) = parent_scope.kind else {
        return;
    };

    let function_type = match function_type::classify(
        name,
        decorator_list,
        parent_scope,
        semantic,
        &checker.settings().pep8_naming.classmethod_decorators,
        &checker.settings().pep8_naming.staticmethod_decorators,
    ) {
        function_type::FunctionType::Function | function_type::FunctionType::StaticMethod => {
            return;
        }
        function_type::FunctionType::Method => match is_metaclass(parent, semantic) {
            IsMetaclass::Yes => FunctionType::ClassMethod,
            IsMetaclass::No => FunctionType::Method,
            IsMetaclass::Maybe => return,
        },
        function_type::FunctionType::ClassMethod => FunctionType::ClassMethod,
        // This violation is caught by `PLW0211` instead
        function_type::FunctionType::NewMethod => {
            return;
        }
    };
    if !checker.is_rule_enabled(function_type.rule()) {
        return;
    }

    let Some(ParameterWithDefault {
        parameter: self_or_cls,
        ..
    }) = parameters
        .posonlyargs
        .first()
        .or_else(|| parameters.args.first())
    else {
        return;
    };

    if &self_or_cls.name == function_type.valid_first_argument_name()
        || checker
            .settings()
            .pep8_naming
            .ignore_names
            .matches(&self_or_cls.name)
    {
        return;
    }

    let mut diagnostic =
        function_type.diagnostic_kind(checker, self_or_cls.name.to_string(), self_or_cls.range());
    diagnostic.try_set_optional_fix(|| {
        rename_parameter(
            checker,
            scope,
            self_or_cls,
            parameters,
            checker.semantic(),
            function_type,
            checker.stylist(),
        )
    });
}

/// Rename the first parameter to `self` or `cls`, if no other parameter has the target name.
fn rename_parameter(
    checker: &Checker,
    scope: &Scope<'_>,
    self_or_cls: &ast::Parameter,
    parameters: &ast::Parameters,
    semantic: &SemanticModel<'_>,
    function_type: FunctionType,
    stylist: &Stylist,
) -> Result<Option<Fix>> {
    // Don't fix if another parameter has the valid name.
    if parameters
        .iter()
        .skip(1)
        .any(|parameter| parameter.name() == function_type.valid_first_argument_name())
    {
        return Ok(None);
    }
    let binding = scope
        .get(&self_or_cls.name)
        .map(|binding_id| semantic.binding(binding_id))
        .unwrap();

    // Don't provide autofix if `self` or `cls` is already defined in the scope.
    if ShadowedKind::new(binding, function_type.valid_first_argument_name(), checker).shadows_any()
    {
        return Ok(None);
    }

    let (edit, rest) = Renamer::rename(
        &self_or_cls.name,
        function_type.valid_first_argument_name(),
        scope,
        semantic,
        stylist,
    )?;
    Ok(Some(Fix::unsafe_edits(edit, rest)))
}
