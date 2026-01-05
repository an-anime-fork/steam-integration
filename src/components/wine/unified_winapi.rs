use std::path::PathBuf;
use std::process::{Output, Child};
use std::ffi::OsStr;

use wincompatlib::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinApiRuntime {
    Default(Proton),
    Wine(Wine)
}

impl From<Wine> for WinApiRuntime {
    #[inline]
    fn from(wine: Wine) -> Self {
        Self::Wine(wine)
    }
}

impl From<Proton> for WinApiRuntime {
    #[inline]
    fn from(proton: Proton) -> Self {
        Self::Default(proton)
    }
}

impl From<WinApiRuntime> for Wine {
    fn from(wine: WinApiRuntime) -> Self {
        match wine {
            WinApiRuntime::Wine(wine) => wine,

            // Kind of unsafe but who cares?
            // I don't like proton anyway lol
            WinApiRuntime::Default(proton) => proton.wine().to_owned()
        }
    }
}

impl WineWithExt for WinApiRuntime {
    #[inline]
    fn with_prefix<T: Into<PathBuf>>(self, prefix: T) -> Self {
        match self {
            Self::Wine(wine) => Self::Wine(wine.with_prefix(prefix)),
            Self::Default(proton) => Self::Default(proton.with_prefix(prefix))
        }
    }

    #[inline]
    fn with_arch(self, arch: WineArch) -> Self {
        match self {
            Self::Wine(wine) => Self::Wine(wine.with_arch(arch)),
            Self::Default(proton) => Self::Default(proton.with_arch(arch))
        }
    }

    #[inline]
    fn with_boot(self, boot: WineBoot) -> Self {
        match self {
            Self::Wine(wine) => Self::Wine(wine.with_boot(boot)),
            Self::Default(proton) => Self::Default(proton.with_boot(boot))
        }
    }

    #[inline]
    fn with_server<T: Into<PathBuf>>(self, server: T) -> Self {
        match self {
            Self::Wine(wine) => Self::Wine(wine.with_server(server)),
            Self::Default(proton) => Self::Default(proton.with_server(server))
        }
    }

    #[inline]
    fn with_loader(self, loader: WineLoader) -> Self {
        match self {
            Self::Wine(wine) => Self::Wine(wine.with_loader(loader)),
            Self::Default(proton) => Self::Default(proton.with_loader(loader))
        }
    }

    #[inline]
    fn with_wine_libs(self, wine_libs: WineSharedLibs) -> Self {
        match self {
            Self::Wine(wine) => Self::Wine(wine.with_wine_libs(wine_libs)),
            Self::Default(proton) => Self::Default(proton.with_wine_libs(wine_libs))
        }
    }

    #[inline]
    fn with_gstreamer_libs(self, gstreamer_libs: GstreamerSharedLibs) -> Self {
        match self {
            Self::Wine(wine) => Self::Wine(wine.with_gstreamer_libs(gstreamer_libs)),
            Self::Default(proton) => Self::Default(proton.with_gstreamer_libs(gstreamer_libs))
        }
    }
}

impl WineBootExt for WinApiRuntime {
    #[inline]
    fn wineboot_command(&self) -> std::process::Command {
        match self {
            Self::Wine(wine) => wine.wineboot_command(),
            Self::Default(proton) => proton.wineboot_command()
        }
    }

    #[inline]
    fn init_prefix(&self, path: Option<impl Into<PathBuf>>) -> anyhow::Result<Output> {
        match self {
            Self::Wine(wine) => wine.init_prefix(path),
            Self::Default(proton) => proton.init_prefix(path)
        }
    }

    #[inline]
    fn update_prefix(&self, path: Option<impl Into<PathBuf>>) -> anyhow::Result<Output> {
        match self {
            Self::Wine(wine) => wine.update_prefix(path),
            Self::Default(proton) => proton.update_prefix(path)
        }
    }

    #[inline]
    fn stop_processes(&self, force: bool) -> anyhow::Result<Output> {
        match self {
            Self::Wine(wine) => wine.stop_processes(force),
            Self::Default(proton) => proton.stop_processes(force)
        }
    }

    #[inline]
    fn restart(&self) -> anyhow::Result<Output> {
        match self {
            Self::Wine(wine) => wine.restart(),
            Self::Default(proton) => proton.restart()
        }
    }

    #[inline]
    fn shutdown(&self) -> anyhow::Result<Output> {
        match self {
            Self::Wine(wine) => wine.shutdown(),
            Self::Default(proton) => proton.shutdown()
        }
    }

    #[inline]
    fn end_session(&self) -> anyhow::Result<Output> {
        match self {
            Self::Wine(wine) => wine.end_session(),
            Self::Default(proton) => proton.end_session()
        }
    }
}

impl WineRunExt for WinApiRuntime {
    #[inline]
    fn run<T: AsRef<OsStr>>(&self, binary: T) -> anyhow::Result<Child> {
        match self {
            Self::Wine(wine) => wine.run(binary),
            Self::Default(proton) => proton.run(binary)
        }
    }

    #[inline]
    fn run_args<T, S>(&self, args: T) -> anyhow::Result<Child>
    where
        T: IntoIterator<Item = S>,
        S: AsRef<OsStr>
    {
        match self {
            Self::Wine(wine) => wine.run_args(args),
            Self::Default(proton) => proton.run_args(args)
        }
    }

    #[inline]
    fn run_args_with_env<T, K, S>(&self, args: T, envs: K) -> anyhow::Result<Child>
    where
        T: IntoIterator<Item = S>,
        K: IntoIterator<Item = (S, S)>,
        S: AsRef<OsStr>
    {
        match self {
            Self::Wine(wine) => wine.run_args_with_env(args, envs),
            Self::Default(proton) => proton.run_args_with_env(args, envs)
        }
    }

    #[inline]
    fn winepath(&self, path: &str) -> anyhow::Result<PathBuf> {
        match self {
            Self::Wine(wine) => wine.winepath(path),
            Self::Default(proton) => proton.winepath(path)
        }
    }
}

impl WineOverridesExt for WinApiRuntime {
    #[inline]
    fn add_override(&self, dll_name: impl AsRef<str>, modes: impl IntoIterator<Item = OverrideMode>) -> anyhow::Result<()> {
        match self {
            Self::Wine(wine) => wine.add_override(dll_name, modes),
            Self::Default(proton) => proton.add_override(dll_name, modes)
        }
    }

    #[inline]
    fn delete_override(&self, dll_name: impl AsRef<str>) -> anyhow::Result<()> {
        match self {
            Self::Wine(wine) => wine.delete_override(dll_name),
            Self::Default(proton) => proton.delete_override(dll_name)
        }
    }
}

impl WineFontsExt for WinApiRuntime {
    #[inline]
    fn register_font(&self, ttf: impl AsRef<str>, font_name: impl AsRef<str>) -> anyhow::Result<()> {
        match self {
            Self::Wine(wine) => wine.register_font(ttf, font_name),
            Self::Default(proton) => proton.register_font(ttf, font_name)
        }
    }

    #[inline]
    fn font_is_installed(&self, ttf: impl AsRef<str>) -> bool {
        match self {
            Self::Wine(wine) => wine.font_is_installed(ttf),
            Self::Default(proton) => proton.font_is_installed(ttf)
        }
    }

    #[inline]
    fn install_font(&self, font: Font) -> anyhow::Result<()> {
        match self {
            Self::Wine(wine) => wine.install_font(font),
            Self::Default(proton) => proton.install_font(font)
        }
    }
}
