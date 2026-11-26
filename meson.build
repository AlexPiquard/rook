project(
  'rook',
  'rust',
  version: '0.1.0',
  meson_version: '>= 1.9.2',
  # license: 'MIT',
)

i18n = import('i18n')
gnome = import('gnome')


base_id = 'fr.alexpiquard.rook'
path_id = '/fr/alexpiquard/rook/'

dependency('glib-2.0', version: '>= 2.86.3')
dependency('gio-2.0', version: '>= 2.86.3')
dependency('gtk4', version: '>= 4.20.3')

find_program('glib-compile-resources')
glib_compile_schemas = find_program('glib-compile-schemas')
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstreamcli = find_program('appstreamcli', required: false)
cargo = find_program('cargo')

name = 'Rook'
version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'
gettext_package = meson.project_name()

if get_option('profile') == 'development'
  profile = 'Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: false).stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format(vcs_tag)
  endif
  application_id = '@0@.@1@'.format(base_id, profile)
else
  profile = ''
  version_suffix = ''
  application_id = base_id
endif

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.project_source_root()
)

if get_option('profile') == 'development'
  # Setup pre-commit hook for ensuring coding style is always consistent
  message('Setting up git pre-commit hook..')
  run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit', check: false)
endif

subdir('data')
subdir('po')

cargo_home = meson.project_build_root() / 'cargo-home'
cargo_target = meson.project_build_root() / 'cargo-target'
cargo_env = environment()
cargo_env.set('CARGO_HOME', cargo_home)
cargo_env.set('CARGO_TARGET_DIR', cargo_target)
cargo_env.set('MESON_APP_NAME', name)
cargo_env.set('MESON_APP_ID', application_id)
cargo_env.set('MESON_PATH_ID', path_id)
cargo_env.set('MESON_GETTEXT_PACKAGE', gettext_package)
cargo_env.set('MESON_LOCALEDIR', localedir)
cargo_env.set('MESON_PKGDATADIR', pkgdatadir)
cargo_env.set('MESON_PROFILE', profile)
cargo_env.set('MESON_VERSION', version + version_suffix)
cargo_env.set('MESON_RESOURCES_FILE', pkgdatadir / 'resources.gresource')

meson.add_devenv(cargo_env)

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]
cargo_options += [ '--package', 'rook-gtk' ]

if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  depends: resources,
  env: cargo_env,
  command: [
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)
