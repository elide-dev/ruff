import subprocess
import sys
from _typeshed import StrOrBytesPath
from asyncio import events, protocols, streams, transports
from collections.abc import Callable, Collection
from typing import IO, Any, Literal

# Keep asyncio.__all__ updated with any changes to __all__ here
__all__ = ("create_subprocess_exec", "create_subprocess_shell")

PIPE: int
STDOUT: int
DEVNULL: int

class SubprocessStreamProtocol(streams.FlowControlMixin, protocols.SubprocessProtocol):
    """Like StreamReaderProtocol, but for a subprocess."""

    stdin: streams.StreamWriter | None
    stdout: streams.StreamReader | None
    stderr: streams.StreamReader | None
    def __init__(self, limit: int, loop: events.AbstractEventLoop) -> None: ...
    def pipe_data_received(self, fd: int, data: bytes | str) -> None: ...

class Process:
    stdin: streams.StreamWriter | None
    stdout: streams.StreamReader | None
    stderr: streams.StreamReader | None
    pid: int
    def __init__(
        self, transport: transports.BaseTransport, protocol: protocols.BaseProtocol, loop: events.AbstractEventLoop
    ) -> None: ...
    @property
    def returncode(self) -> int | None: ...
    async def wait(self) -> int:
        """Wait until the process exit and return the process return code."""

    def send_signal(self, signal: int) -> None: ...
    def terminate(self) -> None: ...
    def kill(self) -> None: ...
    async def communicate(self, input: bytes | bytearray | memoryview | None = None) -> tuple[bytes, bytes]: ...

if sys.version_info >= (3, 11):
    async def create_subprocess_shell(
        cmd: str | bytes,
        stdin: int | IO[Any] | None = None,
        stdout: int | IO[Any] | None = None,
        stderr: int | IO[Any] | None = None,
        limit: int = 65536,
        *,
        # These parameters are forced to these values by BaseEventLoop.subprocess_shell
        universal_newlines: Literal[False] = False,
        shell: Literal[True] = True,
        bufsize: Literal[0] = 0,
        encoding: None = None,
        errors: None = None,
        text: Literal[False] | None = None,
        # These parameters are taken by subprocess.Popen, which this ultimately delegates to
        executable: StrOrBytesPath | None = None,
        preexec_fn: Callable[[], Any] | None = None,
        close_fds: bool = True,
        cwd: StrOrBytesPath | None = None,
        env: subprocess._ENV | None = None,
        startupinfo: Any | None = None,
        creationflags: int = 0,
        restore_signals: bool = True,
        start_new_session: bool = False,
        pass_fds: Collection[int] = ...,
        group: None | str | int = None,
        extra_groups: None | Collection[str | int] = None,
        user: None | str | int = None,
        umask: int = -1,
        process_group: int | None = None,
        pipesize: int = -1,
    ) -> Process: ...
    async def create_subprocess_exec(
        program: StrOrBytesPath,
        *args: StrOrBytesPath,
        stdin: int | IO[Any] | None = None,
        stdout: int | IO[Any] | None = None,
        stderr: int | IO[Any] | None = None,
        limit: int = 65536,
        # These parameters are forced to these values by BaseEventLoop.subprocess_exec
        universal_newlines: Literal[False] = False,
        shell: Literal[False] = False,
        bufsize: Literal[0] = 0,
        encoding: None = None,
        errors: None = None,
        text: Literal[False] | None = None,
        # These parameters are taken by subprocess.Popen, which this ultimately delegates to
        executable: StrOrBytesPath | None = None,
        preexec_fn: Callable[[], Any] | None = None,
        close_fds: bool = True,
        cwd: StrOrBytesPath | None = None,
        env: subprocess._ENV | None = None,
        startupinfo: Any | None = None,
        creationflags: int = 0,
        restore_signals: bool = True,
        start_new_session: bool = False,
        pass_fds: Collection[int] = ...,
        group: None | str | int = None,
        extra_groups: None | Collection[str | int] = None,
        user: None | str | int = None,
        umask: int = -1,
        process_group: int | None = None,
        pipesize: int = -1,
    ) -> Process: ...

elif sys.version_info >= (3, 10):
    async def create_subprocess_shell(
        cmd: str | bytes,
        stdin: int | IO[Any] | None = None,
        stdout: int | IO[Any] | None = None,
        stderr: int | IO[Any] | None = None,
        limit: int = 65536,
        *,
        # These parameters are forced to these values by BaseEventLoop.subprocess_shell
        universal_newlines: Literal[False] = False,
        shell: Literal[True] = True,
        bufsize: Literal[0] = 0,
        encoding: None = None,
        errors: None = None,
        text: Literal[False] | None = None,
        # These parameters are taken by subprocess.Popen, which this ultimately delegates to
        executable: StrOrBytesPath | None = None,
        preexec_fn: Callable[[], Any] | None = None,
        close_fds: bool = True,
        cwd: StrOrBytesPath | None = None,
        env: subprocess._ENV | None = None,
        startupinfo: Any | None = None,
        creationflags: int = 0,
        restore_signals: bool = True,
        start_new_session: bool = False,
        pass_fds: Collection[int] = ...,
        group: None | str | int = None,
        extra_groups: None | Collection[str | int] = None,
        user: None | str | int = None,
        umask: int = -1,
        pipesize: int = -1,
    ) -> Process: ...
    async def create_subprocess_exec(
        program: StrOrBytesPath,
        *args: StrOrBytesPath,
        stdin: int | IO[Any] | None = None,
        stdout: int | IO[Any] | None = None,
        stderr: int | IO[Any] | None = None,
        limit: int = 65536,
        # These parameters are forced to these values by BaseEventLoop.subprocess_exec
        universal_newlines: Literal[False] = False,
        shell: Literal[False] = False,
        bufsize: Literal[0] = 0,
        encoding: None = None,
        errors: None = None,
        text: Literal[False] | None = None,
        # These parameters are taken by subprocess.Popen, which this ultimately delegates to
        executable: StrOrBytesPath | None = None,
        preexec_fn: Callable[[], Any] | None = None,
        close_fds: bool = True,
        cwd: StrOrBytesPath | None = None,
        env: subprocess._ENV | None = None,
        startupinfo: Any | None = None,
        creationflags: int = 0,
        restore_signals: bool = True,
        start_new_session: bool = False,
        pass_fds: Collection[int] = ...,
        group: None | str | int = None,
        extra_groups: None | Collection[str | int] = None,
        user: None | str | int = None,
        umask: int = -1,
        pipesize: int = -1,
    ) -> Process: ...

else:  # >= 3.9
    async def create_subprocess_shell(
        cmd: str | bytes,
        stdin: int | IO[Any] | None = None,
        stdout: int | IO[Any] | None = None,
        stderr: int | IO[Any] | None = None,
        loop: events.AbstractEventLoop | None = None,
        limit: int = 65536,
        *,
        # These parameters are forced to these values by BaseEventLoop.subprocess_shell
        universal_newlines: Literal[False] = False,
        shell: Literal[True] = True,
        bufsize: Literal[0] = 0,
        encoding: None = None,
        errors: None = None,
        text: Literal[False] | None = None,
        # These parameters are taken by subprocess.Popen, which this ultimately delegates to
        executable: StrOrBytesPath | None = None,
        preexec_fn: Callable[[], Any] | None = None,
        close_fds: bool = True,
        cwd: StrOrBytesPath | None = None,
        env: subprocess._ENV | None = None,
        startupinfo: Any | None = None,
        creationflags: int = 0,
        restore_signals: bool = True,
        start_new_session: bool = False,
        pass_fds: Collection[int] = ...,
        group: None | str | int = None,
        extra_groups: None | Collection[str | int] = None,
        user: None | str | int = None,
        umask: int = -1,
    ) -> Process: ...
    async def create_subprocess_exec(
        program: StrOrBytesPath,
        *args: StrOrBytesPath,
        stdin: int | IO[Any] | None = None,
        stdout: int | IO[Any] | None = None,
        stderr: int | IO[Any] | None = None,
        loop: events.AbstractEventLoop | None = None,
        limit: int = 65536,
        # These parameters are forced to these values by BaseEventLoop.subprocess_exec
        universal_newlines: Literal[False] = False,
        shell: Literal[False] = False,
        bufsize: Literal[0] = 0,
        encoding: None = None,
        errors: None = None,
        text: Literal[False] | None = None,
        # These parameters are taken by subprocess.Popen, which this ultimately delegates to
        executable: StrOrBytesPath | None = None,
        preexec_fn: Callable[[], Any] | None = None,
        close_fds: bool = True,
        cwd: StrOrBytesPath | None = None,
        env: subprocess._ENV | None = None,
        startupinfo: Any | None = None,
        creationflags: int = 0,
        restore_signals: bool = True,
        start_new_session: bool = False,
        pass_fds: Collection[int] = ...,
        group: None | str | int = None,
        extra_groups: None | Collection[str | int] = None,
        user: None | str | int = None,
        umask: int = -1,
    ) -> Process: ...
