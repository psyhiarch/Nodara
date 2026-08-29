project('Diagrams', 'rust', version: '0.1.0', meson_version: '>=1.1')

gnome = import('gnome')

base_id = 'org.gtk_rs'
application_id = '@0@.@1@'.format(base_id, meson.project_name())
schema = '@0@.gschema.xml'.format(application_id)

subdir('data')
subdir('src')
