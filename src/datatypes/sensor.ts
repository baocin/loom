import { BaseData, CameraType, ConnectionType } from './types';

// Motion sensors
export class AccelerometerData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    x!: number;
    y!: number;
    z!: number;
    accuracy?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<AccelerometerData>) {
        Object.assign(this, data);
    }
}

export class GyroscopeData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    x!: number;
    y!: number;
    z!: number;
    accuracy?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<GyroscopeData>) {
        Object.assign(this, data);
    }
}

export class MagnetometerData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    x!: number;
    y!: number;
    z!: number;
    accuracy?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<MagnetometerData>) {
        Object.assign(this, data);
    }
}

// Location sensors
export class GPSData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    latitude!: number;
    longitude!: number;
    altitude?: number;
    accuracy?: number;
    speed?: number;
    bearing?: number;
    satellites?: number;
    provider?: string;
    metadata?: Record<string, any>;

    constructor(data: Partial<GPSData>) {
        Object.assign(this, data);
    }
}

// Health sensors
export class HeartRateData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    bpm!: number;
    confidence?: number;
    rr_intervals?: number[];
    metadata?: Record<string, any>;

    constructor(data: Partial<HeartRateData>) {
        Object.assign(this, data);
    }
}

export class ECGData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    voltage!: number[];
    time!: number[];
    rhythm_classification?: string;
    heart_rate?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<ECGData>) {
        Object.assign(this, data);
    }
}

export class BloodOxygenData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    spo2!: number;
    confidence?: number;
    raw_values?: number[];
    metadata?: Record<string, any>;

    constructor(data: Partial<BloodOxygenData>) {
        Object.assign(this, data);
    }
}

export class StressData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    stress_score!: number;
    stress_level?: string;
    hrv?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<StressData>) {
        Object.assign(this, data);
    }
}

// Environmental sensors
export class ProximityData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    distance!: number;
    near!: boolean;
    metadata?: Record<string, any>;

    constructor(data: Partial<ProximityData>) {
        Object.assign(this, data);
    }
}

export class LightData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    lux!: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<LightData>) {
        Object.assign(this, data);
    }
}

export class PressureData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    hectopascals!: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<PressureData>) {
        Object.assign(this, data);
    }
}

export class TemperatureData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    celsius!: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<TemperatureData>) {
        Object.assign(this, data);
    }
}

export class HumidityData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    percentage!: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<HumidityData>) {
        Object.assign(this, data);
    }
}

// Activity sensors
export class StepCountData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    steps!: number;
    activity_type?: string;
    confidence?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<StepCountData>) {
        Object.assign(this, data);
    }
}

// Audio sensors
export class AudioLevelData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    db!: number;
    peak_db?: number;
    volume?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<AudioLevelData>) {
        Object.assign(this, data);
    }
}

// System sensors
export class BatteryData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    percentage!: number;
    charging!: boolean;
    power_source?: string;
    temperature?: number;
    voltage?: number;
    current?: number;
    metadata?: Record<string, any>;

    constructor(data: Partial<BatteryData>) {
        Object.assign(this, data);
    }
}

export class NetworkData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    type!: ConnectionType;
    state?: string;
    strength?: number;
    carrier?: string;
    roaming?: boolean;
    cellular_technology?: string;
    is_metered?: boolean;
    dns_servers?: string[];
    gateway?: string;
    metadata?: Record<string, any>;

    constructor(data: Partial<NetworkData>) {
        Object.assign(this, data);
    }
}

export class ScreenStateData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    screen_on!: boolean;
    brightness?: number;
    orientation?: string;
    metadata?: Record<string, any>;

    constructor(data: Partial<ScreenStateData>) {
        Object.assign(this, data);
    }
}

// Camera and vision sensors
export class CameraData implements BaseData {
    timestamp!: Date;
    device_id!: string;
    camera_type!: CameraType;
    light_level?: number;
    scene_type?: string;
    objects?: Record<string, any>;
    face_detection?: Record<string, any>;
    focus_distance?: number;
    flash_state?: string;
    zoom_level?: number;
    capture_mode?: string;
    metadata?: Record<string, any>;

    constructor(data: Partial<CameraData>) {
        Object.assign(this, data);
    }
} 