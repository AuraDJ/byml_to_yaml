use crate::common::Endianness;
use roead::{byml::Byml, Endian};
use std::{option::Option, path::PathBuf};

xflags::xflags! {
    src "./src/cli.rs"

    cmd byml-to-yaml {

        /// Verbose logging for debugging
        optional -d, --debug

        cmd to-yaml {
            /// Path to the source BYML to convert to YAML
            required path: PathBuf

            /// Output YAML file (defaults to $path with a yml extension)
            optional -o, --output output: PathBuf
        }

        cmd to-byml {
            /// Path to the source YAML to convert to BYML
            required path: PathBuf

            /// Output BYML file (defaults to $path with a byml extension)
            optional -o, --output output: PathBuf

            /// Output BYML version
            optional -v, --version version: u16

            /// Output BYML endianness
            optional -e, --endianness endianness: Endianness
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct BymlToYaml {
    pub debug: bool,
    pub subcommand: BymlToYamlCmd,
}

#[derive(Debug)]
pub enum BymlToYamlCmd {
    ToYaml(ToYaml),
    ToByml(ToByml),
}

#[derive(Debug)]
pub struct ToYaml {
    pub path: PathBuf,

    pub output: Option<PathBuf>,
}

#[derive(Debug)]
pub struct ToByml {
    pub path: PathBuf,

    pub output: Option<PathBuf>,
    pub version: Option<u16>,
    pub endianness: Option<Endianness>,
}

impl BymlToYaml {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end

pub struct Runner {
    cli: BymlToYaml,
}

impl Runner {
    pub fn new(cli: BymlToYaml) -> Self {
        return Self { cli };
    }

    fn to_byml(
        &self,
        path: &mut PathBuf,
        output: &Option<PathBuf>,
        version: &Option<u16>,
        endian: Endian,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let text = std::fs::read_to_string(path.as_path())?;
        let byml = Byml::from_text(text)?;
        std::fs::write(
            match output {
                None => {
                    path.set_extension("byml");
                    path
                }
                Some(output_path) => output_path,
            },
            byml.to_binary_with_version(
                endian,
                match version {
                    None => 4,
                    Some(value) => *value,
                },
            ),
        )?;

        return Ok(());
    }

    fn to_yaml(
        &self,
        path: &mut PathBuf,
        output: &Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = std::fs::read(path.as_path())?;
        let byml = Byml::from_binary(buffer)?;
        std::fs::write(
            match output {
                None => {
                    path.set_extension("yml");
                    path
                }
                Some(output_path) => output_path,
            },
            byml.to_text(),
        )?;

        return Ok(());
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.cli.subcommand {
            BymlToYamlCmd::ToByml(ToByml {
                path,
                output,
                version,
                endianness,
            }) => {
                return self.to_byml(
                    &mut path.to_path_buf(),
                    output,
                    version,
                    match endianness {
                        None => Endian::Little,
                        Some(x) => match x {
                            Endianness::Big => Endian::Big,
                            Endianness::Little => Endian::Little,
                        },
                    },
                );
            }
            BymlToYamlCmd::ToYaml(ToYaml { path, output }) => {
                return self.to_yaml(&mut path.to_path_buf(), output);
            }
        }
    }
}
