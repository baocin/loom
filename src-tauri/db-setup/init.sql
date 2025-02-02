
-- User Management Tables
CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    name VARCHAR,
    encrypted_password VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE oauth_accounts (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES users(id),
    provider VARCHAR NOT NULL,
    provider_user_id VARCHAR NOT NULL,
    access_token VARCHAR NOT NULL,
    refresh_token VARCHAR,
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(provider, provider_user_id)
);

CREATE TABLE sessions (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES users(id),
    token VARCHAR NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Device Data Tables
CREATE TYPE device_type AS ENUM (
    'UNKNOWN',
    'HEADPHONE',
    'SPEAKER', 
    'CAR',
    'KEYBOARD',
    'MOUSE',
    'GAMEPAD',
    'WATCH',
    'PHONE'
);

CREATE TYPE camera_type AS ENUM (
    'UNKNOWN',
    'FRONT',
    'BACK_MAIN',
    'BACK_WIDE',
    'BACK_TELEPHOTO'
);

CREATE TYPE connection_type AS ENUM (
    'UNKNOWN',
    'NONE', 
    'WIFI',
    'CELLULAR_2G',
    'CELLULAR_3G', 
    'CELLULAR_4G',
    'CELLULAR_5G',
    'ETHERNET',
    'VPN'
);

CREATE TYPE entity_type AS ENUM (
    'FACE',
    'OBJECT',
    'POSE',
    'AUDIO'
);

CREATE TYPE note_priority AS ENUM (
    'LOW',
    'MEDIUM',
    'HIGH',
    'CRITICAL'
);

CREATE TABLE devices (
    device_id VARCHAR PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES users(id),
    device_type VARCHAR NOT NULL,
    os_type VARCHAR NOT NULL,
    os_version VARCHAR NOT NULL,
    app_version VARCHAR NOT NULL,
    available_sensors VARCHAR[],
    capabilities JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Sensor Data Tables
CREATE TABLE accelerometer_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    x FLOAT,
    y FLOAT,
    z FLOAT,
    accuracy FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE gyroscope_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    x FLOAT,
    y FLOAT,
    z FLOAT,
    accuracy FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE magnetometer_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    x FLOAT,
    y FLOAT,
    z FLOAT,
    accuracy FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE gps_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    altitude DOUBLE PRECISION,
    accuracy FLOAT,
    speed FLOAT,
    bearing FLOAT,
    satellites INTEGER,
    provider VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE heart_rate_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    bpm INTEGER,
    confidence FLOAT,
    rr_intervals FLOAT[],
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE proximity_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    distance FLOAT,
    near BOOLEAN,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE light_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    lux FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE pressure_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    hectopascals FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE temperature_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    celsius FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE humidity_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    percentage FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE step_count_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    steps INTEGER,
    activity_type VARCHAR,
    confidence FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE call_log_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    call_type VARCHAR NOT NULL,
    phone_number VARCHAR,
    contact_name VARCHAR,
    duration_seconds INTEGER,
    is_missed BOOLEAN,
    is_blocked BOOLEAN,
    sim_slot INTEGER,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE todos_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    todo_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    due_date TIMESTAMP,
    completed BOOLEAN DEFAULT FALSE,
    completed_at TIMESTAMP,
    priority INTEGER,
    tags VARCHAR[],
    metadata JSON,
    PRIMARY KEY (timestamp, device_id, todo_id)
);


CREATE TABLE audio_level_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    db FLOAT,
    peak_db FLOAT,
    volume FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE battery_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    percentage INTEGER,
    charging BOOLEAN,
    power_source VARCHAR,
    temperature INTEGER,
    voltage INTEGER,
    current INTEGER,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE network_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    type connection_type,
    state VARCHAR,
    strength INTEGER,
    carrier VARCHAR,
    roaming BOOLEAN,
    cellular_technology VARCHAR,
    is_metered BOOLEAN,
    dns_servers VARCHAR[],
    gateway VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE screen_state_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    screen_on BOOLEAN,
    brightness INTEGER,
    orientation VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE notification_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    package_name VARCHAR,
    title VARCHAR,
    priority INTEGER,
    category VARCHAR,
    posted_at TIMESTAMP,
    removed_at TIMESTAMP,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE app_usage_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    package_name VARCHAR,
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    activity_type VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE wifi_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    ssid VARCHAR,
    bssid VARCHAR,
    strength INTEGER,
    frequency INTEGER,
    ip_address VARCHAR,
    link_speed INTEGER,
    security_type VARCHAR,
    is_5ghz BOOLEAN,
    is_6ghz BOOLEAN,
    is_passpoint BOOLEAN,
    is_restricted BOOLEAN,
    nearby_networks JSON,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE bluetooth_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    nearby_devices JSON,
    connected_devices JSON,
    enabled BOOLEAN,
    discovering BOOLEAN,
    local_name VARCHAR,
    local_address VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE camera_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    camera_type camera_type,
    light_level INTEGER,
    scene_type VARCHAR,
    objects JSON,
    face_detection JSON,
    focus_distance FLOAT,
    flash_state VARCHAR,
    zoom_level FLOAT,
    capture_mode VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE microphone_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    average_frequency FLOAT,
    dominant_frequency FLOAT,
    raw_output VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE app_event_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    package_name VARCHAR,
    event_type VARCHAR,
    activity_name VARCHAR,
    process_state JSON,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE system_audio_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    raw_output VARCHAR,
    volume_level FLOAT,
    audio_output VARCHAR,
    is_music_playing BOOLEAN,
    active_media_app VARCHAR,
    active_streams JSON,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE network_speed_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    download_mbps FLOAT,
    upload_mbps FLOAT,
    download_packets_lost INTEGER,
    upload_packets_lost INTEGER,
    jitter_ms FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE server_latency_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    ping_ms FLOAT,
    websocket_latency_ms FLOAT,
    packet_loss_percentage INTEGER,
    historic_pings FLOAT[],
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE skin_temperature_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    celsius FLOAT,
    accuracy FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE ecg_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    voltage FLOAT[],
    time FLOAT[],
    rhythm_classification VARCHAR,
    heart_rate FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE blood_oxygen_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    spo2 INTEGER,
    confidence FLOAT,
    raw_values FLOAT[],
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE stress_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    stress_score INTEGER,
    stress_level VARCHAR,
    hrv FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE compass_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    heading FLOAT,
    accuracy FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE screen_details_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    screen_on BOOLEAN,
    brightness_level INTEGER,
    brightness_nits FLOAT,
    auto_brightness BOOLEAN,
    night_mode BOOLEAN,
    display_mode VARCHAR,
    refresh_rate INTEGER,
    width INTEGER,
    height INTEGER,
    density FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE object_detection_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    objects JSON,
    source camera_type,
    frame_timestamp TIMESTAMP,
    model_version VARCHAR,
    inference_time_ms FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE face_recognition_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    faces JSON,
    source camera_type,
    frame_timestamp TIMESTAMP,
    model_version VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE pose_detection_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    poses JSON,
    source camera_type,
    frame_timestamp TIMESTAMP,
    model_version VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE known_entities (
    entity_id VARCHAR PRIMARY KEY,
    type entity_type NOT NULL,
    label VARCHAR NOT NULL,
    embedding FLOAT[1024],
    metadata JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Notes Tables
CREATE TABLE notes (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES users(id),
    timestamp TIMESTAMP NOT NULL,
    content TEXT NOT NULL,
    priority note_priority DEFAULT 'MEDIUM',
    parent_id VARCHAR REFERENCES notes(id),
    tags VARCHAR[],
    embedding FLOAT[1024],
    metadata JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE note_references (
    note_id VARCHAR NOT NULL REFERENCES notes(id),
    reference_type VARCHAR NOT NULL,
    reference_id VARCHAR NOT NULL,
    timestamp TIMESTAMP,
    metadata JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (note_id, reference_type, reference_id)
);

-- Indexes for performance
CREATE INDEX idx_accelerometer_device_time ON accelerometer_data(device_id, timestamp);
CREATE INDEX idx_gyroscope_device_time ON gyroscope_data(device_id, timestamp);
CREATE INDEX idx_magnetometer_device_time ON magnetometer_data(device_id, timestamp);
CREATE INDEX idx_gps_device_time ON gps_data(device_id, timestamp);
CREATE INDEX idx_heart_rate_device_time ON heart_rate_data(device_id, timestamp);
CREATE INDEX idx_proximity_device_time ON proximity_data(device_id, timestamp);
CREATE INDEX idx_light_device_time ON light_data(device_id, timestamp);
CREATE INDEX idx_pressure_device_time ON pressure_data(device_id, timestamp);
CREATE INDEX idx_temperature_device_time ON temperature_data(device_id, timestamp);
CREATE INDEX idx_humidity_device_time ON humidity_data(device_id, timestamp);
CREATE INDEX idx_step_count_device_time ON step_count_data(device_id, timestamp);
CREATE INDEX idx_audio_level_device_time ON audio_level_data(device_id, timestamp);
CREATE INDEX idx_battery_device_time ON battery_data(device_id, timestamp);
CREATE INDEX idx_network_device_time ON network_data(device_id, timestamp);
CREATE INDEX idx_screen_state_device_time ON screen_state_data(device_id, timestamp);
CREATE INDEX idx_notification_device_time ON notification_data(device_id, timestamp);
CREATE INDEX idx_app_usage_device_time ON app_usage_data(device_id, timestamp);
CREATE INDEX idx_wifi_device_time ON wifi_data(device_id, timestamp);
CREATE INDEX idx_bluetooth_device_time ON bluetooth_data(device_id, timestamp);
CREATE INDEX idx_camera_device_time ON camera_data(device_id, timestamp);
CREATE INDEX idx_microphone_device_time ON microphone_data(device_id, timestamp);
CREATE INDEX idx_app_event_device_time ON app_event_data(device_id, timestamp);
CREATE INDEX idx_system_audio_device_time ON system_audio_data(device_id, timestamp);
CREATE INDEX idx_network_speed_device_time ON network_speed_data(device_id, timestamp);
CREATE INDEX idx_server_latency_device_time ON server_latency_data(device_id, timestamp);
CREATE INDEX idx_skin_temperature_device_time ON skin_temperature_data(device_id, timestamp);
CREATE INDEX idx_ecg_device_time ON ecg_data(device_id, timestamp);
CREATE INDEX idx_blood_oxygen_device_time ON blood_oxygen_data(device_id, timestamp);
CREATE INDEX idx_stress_device_time ON stress_data(device_id, timestamp);
CREATE INDEX idx_compass_device_time ON compass_data(device_id, timestamp);
CREATE INDEX idx_screen_details_device_time ON screen_details_data(device_id, timestamp);
CREATE INDEX idx_object_detection_device_time ON object_detection_data(device_id, timestamp);
CREATE INDEX idx_face_recognition_device_time ON face_recognition_data(device_id, timestamp);
CREATE INDEX idx_pose_detection_device_time ON pose_detection_data(device_id, timestamp);

CREATE INDEX idx_oauth_user ON oauth_accounts(user_id);
CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_devices_user ON devices(user_id);
CREATE INDEX idx_entities_type ON known_entities(type);
CREATE INDEX idx_notes_user ON notes(user_id);
CREATE INDEX idx_notes_timestamp ON notes(timestamp);
CREATE INDEX idx_notes_tags ON notes USING GIN (tags);
CREATE INDEX idx_note_refs_timestamp ON note_references(timestamp);

-- Vector similarity search indexes
CREATE INDEX idx_entity_embedding ON known_entities USING HNSW (embedding);
CREATE INDEX idx_notes_embedding ON notes USING HNSW (embedding);
    cellular_technology VARCHAR,
    is_metered BOOLEAN,
    dns_servers VARCHAR[],
    gateway VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE screen_state_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    screen_on BOOLEAN,
    brightness INTEGER,
    orientation VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE notification_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    package_name VARCHAR,
    title VARCHAR,
    priority INTEGER,
    category VARCHAR,
    posted_at TIMESTAMP,
    removed_at TIMESTAMP,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE app_usage_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    package_name VARCHAR,
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    activity_type VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE wifi_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    ssid VARCHAR,
    bssid VARCHAR,
    strength INTEGER,
    frequency INTEGER,
    ip_address VARCHAR,
    link_speed INTEGER,
    security_type VARCHAR,
    is_5ghz BOOLEAN,
    is_6ghz BOOLEAN,
    is_passpoint BOOLEAN,
    is_restricted BOOLEAN,
    nearby_networks JSON,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE bluetooth_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    nearby_devices JSON,
    connected_devices JSON,
    enabled BOOLEAN,
    discovering BOOLEAN,
    local_name VARCHAR,
    local_address VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE camera_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    camera_type camera_type,
    light_level INTEGER,
    scene_type VARCHAR,
    objects JSON,
    face_detection JSON,
    focus_distance FLOAT,
    flash_state VARCHAR,
    zoom_level FLOAT,
    capture_mode VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE microphone_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    average_frequency FLOAT,
    dominant_frequency FLOAT,
    raw_output VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE app_event_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    package_name VARCHAR,
    event_type VARCHAR,
    activity_name VARCHAR,
    process_state JSON,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE system_audio_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    raw_output VARCHAR,
    volume_level FLOAT,
    audio_output VARCHAR,
    is_music_playing BOOLEAN,
    active_media_app VARCHAR,
    active_streams JSON,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE network_speed_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    download_mbps FLOAT,
    upload_mbps FLOAT,
    download_packets_lost INTEGER,
    upload_packets_lost INTEGER,
    jitter_ms FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE server_latency_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    ping_ms FLOAT,
    websocket_latency_ms FLOAT,
    packet_loss_percentage INTEGER,
    historic_pings FLOAT[],
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE skin_temperature_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    celsius FLOAT,
    accuracy FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE ecg_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    voltage FLOAT[],
    time FLOAT[],
    rhythm_classification VARCHAR,
    heart_rate FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE blood_oxygen_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    spo2 INTEGER,
    confidence FLOAT,
    raw_values FLOAT[],
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE stress_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    stress_score INTEGER,
    stress_level VARCHAR,
    hrv FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE compass_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    heading FLOAT,
    accuracy FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE screen_details_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    screen_on BOOLEAN,
    brightness_level INTEGER,
    brightness_nits FLOAT,
    auto_brightness BOOLEAN,
    night_mode BOOLEAN,
    display_mode VARCHAR,
    refresh_rate INTEGER,
    width INTEGER,
    height INTEGER,
    density FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE object_detection_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    objects JSON,
    source camera_type,
    frame_timestamp TIMESTAMP,
    model_version VARCHAR,
    inference_time_ms FLOAT,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE face_recognition_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    faces JSON,
    source camera_type,
    frame_timestamp TIMESTAMP,
    model_version VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE pose_detection_data (
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR NOT NULL REFERENCES devices(device_id),
    poses JSON,
    source camera_type,
    frame_timestamp TIMESTAMP,
    model_version VARCHAR,
    metadata JSON,
    PRIMARY KEY (timestamp, device_id)
);

CREATE TABLE known_entities (
    entity_id VARCHAR PRIMARY KEY,
    type entity_type NOT NULL,
    label VARCHAR NOT NULL,
    embedding FLOAT[1024],
    metadata JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Notes Tables
CREATE TABLE notes (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES users(id),
    timestamp TIMESTAMP NOT NULL,
    content TEXT NOT NULL,
    priority note_priority DEFAULT 'MEDIUM',
    parent_id VARCHAR REFERENCES notes(id),
    tags VARCHAR[],
    embedding FLOAT[1024],
    metadata JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE note_references (
    note_id VARCHAR NOT NULL REFERENCES notes(id),
    reference_type VARCHAR NOT NULL,
    reference_id VARCHAR NOT NULL,
    timestamp TIMESTAMP,
    metadata JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (note_id, reference_type, reference_id)
);

-- Indexes for performance
CREATE INDEX idx_accelerometer_device_time ON accelerometer_data(device_id, timestamp);
CREATE INDEX idx_gyroscope_device_time ON gyroscope_data(device_id, timestamp);
CREATE INDEX idx_magnetometer_device_time ON magnetometer_data(device_id, timestamp);
CREATE INDEX idx_gps_device_time ON gps_data(device_id, timestamp);
CREATE INDEX idx_heart_rate_device_time ON heart_rate_data(device_id, timestamp);
CREATE INDEX idx_proximity_device_time ON proximity_data(device_id, timestamp);
CREATE INDEX idx_light_device_time ON light_data(device_id, timestamp);
CREATE INDEX idx_pressure_device_time ON pressure_data(device_id, timestamp);
CREATE INDEX idx_temperature_device_time ON temperature_data(device_id, timestamp);
CREATE INDEX idx_humidity_device_time ON humidity_data(device_id, timestamp);
CREATE INDEX idx_step_count_device_time ON step_count_data(device_id, timestamp);
CREATE INDEX idx_audio_level_device_time ON audio_level_data(device_id, timestamp);
CREATE INDEX idx_battery_device_time ON battery_data(device_id, timestamp);
CREATE INDEX idx_network_device_time ON network_data(device_id, timestamp);
CREATE INDEX idx_screen_state_device_time ON screen_state_data(device_id, timestamp);
CREATE INDEX idx_notification_device_time ON notification_data(device_id, timestamp);
CREATE INDEX idx_app_usage_device_time ON app_usage_data(device_id, timestamp);
CREATE INDEX idx_wifi_device_time ON wifi_data(device_id, timestamp);
CREATE INDEX idx_bluetooth_device_time ON bluetooth_data(device_id, timestamp);
CREATE INDEX idx_camera_device_time ON camera_data(device_id, timestamp);
CREATE INDEX idx_microphone_device_time ON microphone_data(device_id, timestamp);
CREATE INDEX idx_app_event_device_time ON app_event_data(device_id, timestamp);
CREATE INDEX idx_system_audio_device_time ON system_audio_data(device_id, timestamp);
CREATE INDEX idx_network_speed_device_time ON network_speed_data(device_id, timestamp);
CREATE INDEX idx_server_latency_device_time ON server_latency_data(device_id, timestamp);
CREATE INDEX idx_skin_temperature_device_time ON skin_temperature_data(device_id, timestamp);
CREATE INDEX idx_ecg_device_time ON ecg_data(device_id, timestamp);
CREATE INDEX idx_blood_oxygen_device_time ON blood_oxygen_data(device_id, timestamp);
CREATE INDEX idx_stress_device_time ON stress_data(device_id, timestamp);
CREATE INDEX idx_compass_device_time ON compass_data(device_id, timestamp);
CREATE INDEX idx_screen_details_device_time ON screen_details_data(device_id, timestamp);
CREATE INDEX idx_object_detection_device_time ON object_detection_data(device_id, timestamp);
CREATE INDEX idx_face_recognition_device_time ON face_recognition_data(device_id, timestamp);
CREATE INDEX idx_pose_detection_device_time ON pose_detection_data(device_id, timestamp);

CREATE INDEX idx_oauth_user ON oauth_accounts(user_id);
CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_devices_user ON devices(user_id);
CREATE INDEX idx_entities_type ON known_entities(type);
CREATE INDEX idx_notes_user ON notes(user_id);
CREATE INDEX idx_notes_timestamp ON notes(timestamp);
CREATE INDEX idx_notes_tags ON notes USING GIN (tags);
CREATE INDEX idx_note_refs_timestamp ON note_references(timestamp);

-- Vector similarity search indexes
CREATE INDEX idx_entity_embedding ON known_entities USING HNSW (embedding);
CREATE INDEX idx_notes_embedding ON notes USING HNSW (embedding);
CREATE INDEX idx_notes_embedding ON notes USING HNSW (embedding);


-- Pruning and Sync Configuration Types
CREATE TYPE sync_priority AS ENUM (
    'CRITICAL',    -- Immediate sync (notes, important events)
    'HIGH',        -- Next sync cycle (health data)
    'MEDIUM',      -- Within hour (GPS, activity)
    'LOW',         -- Daily (historical sensor data)
    'BACKGROUND'   -- When convenient (analytics)
);

CREATE TYPE compression_algorithm AS ENUM (
    'NONE',
    'LZ4',
    'ZSTD',
    'GZIP'
);

-- Data Retention Configuration Table
CREATE TABLE retention_config (
    table_name VARCHAR PRIMARY KEY,
    compression_enabled BOOLEAN NOT NULL DEFAULT false,
    compression_algorithm compression_algorithm DEFAULT 'NONE',
    retention_days INTEGER,
    downsample_after_days INTEGER,
    downsample_ratio INTEGER,
    convert_to_text BOOLEAN DEFAULT false,
    min_required_space_mb INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata JSON,
    CONSTRAINT valid_retention CHECK (retention_days > 0),
    CONSTRAINT valid_downsample CHECK (downsample_ratio > 0)
);

-- Sync Priority Configuration Table
CREATE TABLE sync_priorities (
    table_name VARCHAR PRIMARY KEY,
    priority sync_priority NOT NULL,
    batch_size INTEGER DEFAULT 1000,
    max_delay_seconds INTEGER,
    retry_count INTEGER DEFAULT 3,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata JSON,
    CONSTRAINT valid_batch_size CHECK (batch_size > 0),
    CONSTRAINT valid_retry_count CHECK (retry_count >= 0)
);

-- Insert default retention policies
INSERT INTO retention_config 
(table_name, compression_enabled, retention_days, downsample_after_days, downsample_ratio)
VALUES
-- Critical data (no compression, no downsampling)
('notes', false, 3650, null, null),
('heart_rate_data', false, 3650, null, null),
('blood_oxygen_data', false, 3650, null, null),
('ecg_data', true, 3650, 30, 2),

-- High frequency sensor data
('accelerometer_data', true, 30, 7, 10),
('gyroscope_data', true, 30, 7, 10),
('magnetometer_data', true, 30, 7, 10),
('audio_level_data', true, 30, 7, 5),

-- Medium priority data
('gps_data', true, 90, 30, 4),
('step_count_data', true, 365, 30, 2),
('camera_data', true, 90, 30, null),
('microphone_data', true, 90, 30, null),

-- System metrics
('battery_data', true, 90, 7, 6),
('network_data', true, 90, 7, 6),
('screen_state_data', true, 90, 7, 6),
('app_usage_data', true, 90, 7, null);

-- Insert default sync priorities
INSERT INTO sync_priorities 
(table_name, priority, batch_size, max_delay_seconds)
VALUES
-- Critical - Immediate sync
('notes', 'CRITICAL', 1, 0),
('heart_rate_data', 'CRITICAL', 10, 5),
('blood_oxygen_data', 'CRITICAL', 10, 5),
('stress_data', 'CRITICAL', 10, 5),

-- High - Next sync cycle
('ecg_data', 'HIGH', 100, 60),
('app_event_data', 'HIGH', 100, 60),
('notification_data', 'HIGH', 100, 60),

-- Medium - Within hour
('gps_data', 'MEDIUM', 1000, 3600),
('step_count_data', 'MEDIUM', 1000, 3600),
('camera_data', 'MEDIUM', 100, 3600),
('microphone_data', 'MEDIUM', 100, 3600),

-- Low - Daily
('accelerometer_data', 'LOW', 5000, 86400),
('gyroscope_data', 'LOW', 5000, 86400),
('magnetometer_data', 'LOW', 5000, 86400),
('audio_level_data', 'LOW', 1000, 86400),

-- Background - When convenient
('battery_data', 'BACKGROUND', 1000, 172800),
('network_data', 'BACKGROUND', 1000, 172800),
('screen_state_data', 'BACKGROUND', 1000, 172800),
('wifi_data', 'BACKGROUND', 1000, 172800);

-- Create indexes for performance
CREATE INDEX idx_retention_updated ON retention_config(updated_at);
CREATE INDEX idx_sync_priority ON sync_priorities(priority);
