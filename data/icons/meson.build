application_id = meson.project_name()

# Install our icons in all the required sizes
icon_sizes = ['16', '24', '32', '48', '64', '128']

foreach i : icon_sizes
    install_data(
        'hicolor' / i + '.svg',
        install_dir: get_option('datadir') / 'icons' / 'hicolor' / i + 'x' + i / 'apps',
        rename: application_id + '.svg'
    )
    install_data(
        'hicolor' / i + '.svg',
        install_dir: get_option('datadir') / 'icons' / 'hicolor' / i + 'x' + i + '@2' / 'apps',
        rename: application_id + '.svg'
    )
endforeach

symbolic_dir = join_paths('hicolor', 'symbolic', 'apps')
install_data(
  join_paths('symbolic.svg'),
  install_dir: join_paths(get_option('datadir'), 'icons', symbolic_dir),
  rename: ('@0@-symbolic.svg').format(application_id)
)
