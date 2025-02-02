// Enums
export enum DeviceType {
    UNKNOWN = 'UNKNOWN',
    HEADPHONE = 'HEADPHONE',
    SPEAKER = 'SPEAKER',
    CAR = 'CAR',
    KEYBOARD = 'KEYBOARD',
    MOUSE = 'MOUSE',
    GAMEPAD = 'GAMEPAD',
    WATCH = 'WATCH',
    PHONE = 'PHONE'
}

export enum CameraType {
    UNKNOWN = 'UNKNOWN',
    FRONT = 'FRONT',
    BACK_MAIN = 'BACK_MAIN',
    BACK_WIDE = 'BACK_WIDE',
    BACK_TELEPHOTO = 'BACK_TELEPHOTO'
}

export enum ConnectionType {
    UNKNOWN = 'UNKNOWN',
    NONE = 'NONE',
    WIFI = 'WIFI',
    CELLULAR_2G = 'CELLULAR_2G',
    CELLULAR_3G = 'CELLULAR_3G',
    CELLULAR_4G = 'CELLULAR_4G',
    CELLULAR_5G = 'CELLULAR_5G',
    ETHERNET = 'ETHERNET',
    VPN = 'VPN'
}

export enum EntityType {
    FACE = 'FACE',
    OBJECT = 'OBJECT',
    POSE = 'POSE',
    AUDIO = 'AUDIO'
}

export enum NotePriority {
    LOW = 'LOW',
    MEDIUM = 'MEDIUM',
    HIGH = 'HIGH',
    CRITICAL = 'CRITICAL'
}

export enum SyncPriority {
    CRITICAL = 'CRITICAL',
    HIGH = 'HIGH',
    MEDIUM = 'MEDIUM',
    LOW = 'LOW',
    BACKGROUND = 'BACKGROUND'
}

export enum CompressionAlgorithm {
    NONE = 'NONE',
    LZ4 = 'LZ4',
    ZSTD = 'ZSTD',
    GZIP = 'GZIP'
}

// Base interfaces
export interface BaseData {
    timestamp: Date;
    device_id: string;
    metadata?: Record<string, any>;
}

export interface BaseEntity {
    created_at: Date;
    updated_at: Date;
} 