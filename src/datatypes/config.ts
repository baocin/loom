import { BaseEntity, CompressionAlgorithm, SyncPriority } from './types';

export class RetentionConfig implements BaseEntity {
    table_name!: string;
    compression_enabled: boolean = false;
    compression_algorithm: CompressionAlgorithm = CompressionAlgorithm.NONE;
    retention_days?: number;
    downsample_after_days?: number;
    downsample_ratio?: number;
    convert_to_text: boolean = false;
    min_required_space_mb?: number;
    created_at!: Date;
    updated_at!: Date;
    metadata?: Record<string, any>;

    constructor(data: Partial<RetentionConfig>) {
        Object.assign(this, data);
    }
}

export class SyncPriorityConfig implements BaseEntity {
    table_name!: string;
    priority!: SyncPriority;
    batch_size: number = 1000;
    max_delay_seconds?: number;
    retry_count: number = 3;
    created_at!: Date;
    updated_at!: Date;
    metadata?: Record<string, any>;

    constructor(data: Partial<SyncPriorityConfig>) {
        Object.assign(this, data);
    }
} 