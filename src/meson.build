simpleapplication_sources = [
  'main.c',
  'simpleapplication-application.c',
  'simpleapplication-window.c',
  'simpleapplication-preferences.c',
]

simpleapplication_deps = [
  dependency('gtk4'),
  dependency('libadwaita-1', version: '>= 1.4'),
]

simpleapplication_sources += gnome.compile_resources('simpleapplication-resources',
  'simpleapplication.gresource.xml',
  c_name: 'simpleapplication'
)

executable('simpleapplication', simpleapplication_sources,
  dependencies: simpleapplication_deps,
       install: true,
       link_args : ['-rdynamic'],
)
