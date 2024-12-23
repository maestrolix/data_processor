CREATE EXTENSION vector;

CREATE TABLE plugin_types (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    description TEXT,
    key VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE data_types (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    description TEXT,
    key VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE plugin_types_data_types (
    id SERIAL PRIMARY KEY,
    is_input BOOLEAN NOT NULL,
    is_required BOOLEAN NOT NULL,
    plugin_type_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_types_plugin_types_data_types FOREIGN KEY (plugin_type_id) REFERENCES plugin_types (id),
    data_type_id INTEGER NOT NULL,
    CONSTRAINT fk_data_types_plugin_types_data_types FOREIGN KEY (data_type_id) REFERENCES data_types (id)
);

CREATE TABLE pipelines (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) UNIQUE NOT NULL,
    description TEXT
);

CREATE TABLE plugin_data_links (
    id SERIAL PRIMARY KEY,
    plugin_type_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_types_plugin_data_links FOREIGN KEY (plugin_type_id) REFERENCES plugin_types (id)
);

CREATE TABLE output_fields (
    id SERIAL PRIMARY KEY,
    plugin_data_links_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_data_links_output_fields FOREIGN KEY (plugin_data_links_id) REFERENCES plugin_data_links (id),
    data_type_id INTEGER NOT NULL,
    CONSTRAINT fk_data_types_output_fields FOREIGN KEY (data_type_id) REFERENCES data_types (id)
);

CREATE TABLE input_fields (
    id SERIAL PRIMARY KEY,
    plugin_data_links_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_data_links_input_fields FOREIGN KEY (plugin_data_links_id) REFERENCES plugin_data_links (id),
    data_type_id INTEGER NOT NULL,
    CONSTRAINT fk_data_types_input_fields FOREIGN KEY (data_type_id) REFERENCES data_types (id),
    from_output_field_id INTEGER NOT NULL,
    CONSTRAINT fk_output_fields_input_fields FOREIGN KEY (from_output_field_id) REFERENCES output_fields (id)
);

CREATE TABLE plugin_facial_detectings (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    plugin_data_links_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_data_links_plugin_facial_detectings FOREIGN KEY (plugin_data_links_id) REFERENCES plugin_data_links (id),
    pipeline_id INTEGER NOT NULL,
    CONSTRAINT fk_pipelines_plugin_facial_detectings FOREIGN KEY (pipeline_id) REFERENCES pipelines (id)
);

CREATE TABLE plugin_facial_recognitions (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    plugin_data_links_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_data_links_plugin_facial_recognitions FOREIGN KEY (plugin_data_links_id) REFERENCES plugin_data_links (id),
    pipeline_id INTEGER NOT NULL,
    CONSTRAINT fk_pipelines_plugin_facial_recognitions FOREIGN KEY (pipeline_id) REFERENCES pipelines (id)
);

CREATE TABLE plugin_image_recognitions (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    plugin_data_links_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_data_links_plugin_image_recognitions FOREIGN KEY (plugin_data_links_id) REFERENCES plugin_data_links (id),
    pipeline_id INTEGER NOT NULL,
    CONSTRAINT fk_pipelines_plugin_image_recognitions FOREIGN KEY (pipeline_id) REFERENCES pipelines (id)
);

CREATE TABLE plugin_decoder (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    plugin_data_links_id INTEGER NOT NULL,
    CONSTRAINT fk_plugin_data_links_plugin_decoder FOREIGN KEY (plugin_data_links_id) REFERENCES plugin_data_links (id),
    pipeline_id INTEGER NOT NULL,
    CONSTRAINT fk_pipelines_plugin_decoder FOREIGN KEY (pipeline_id) REFERENCES pipelines (id)
);
