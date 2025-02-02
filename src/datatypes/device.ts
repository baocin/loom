import { BaseEntity, DeviceType } from './types';

export class Device implements BaseEntity {
    device_id!: string;
    user_id!: string;
    device_type!: DeviceType;
    os_type!: string;
    os_version!: string;
    app_version!: string;
    available_sensors!: string[];
    capabilities!: Record<string, any>;
    created_at!: Date;
    last_seen!: Date;
    updated_at!: Date;

    constructor(data: Partial<Device>) {
        Object.assign(this, data);
    }
}

export interface DeviceCapabilities {
    hasCamera: boolean;
    hasMicrophone: boolean;
    hasGPS: boolean;
    hasAccelerometer: boolean;
    hasGyroscope: boolean;
    hasMagnetometer: boolean;
    hasProximity: boolean;
    hasLight: boolean;
    hasPressure: boolean;
    hasTemperature: boolean;
    hasHumidity: boolean;
    hasStepCounter: boolean;
    hasHeartRate: boolean;
    hasECG: boolean;
    hasBloodOxygen: boolean;
    hasStress: boolean;
    hasCompass: boolean;
    screenDetails: {
        width: number;
        height: number;
        density: number;
        refreshRate: number;
    };
} 