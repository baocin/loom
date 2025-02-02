import { BaseEntity, EntityType, NotePriority } from './types';

export class Note implements BaseEntity {
    id!: string;
    user_id!: string;
    timestamp!: Date;
    content!: string;
    priority: NotePriority = NotePriority.MEDIUM;
    parent_id?: string;
    tags?: string[];
    embedding?: number[];
    metadata?: Record<string, any>;
    created_at!: Date;
    updated_at!: Date;

    constructor(data: Partial<Note>) {
        Object.assign(this, data);
    }
}

export class NoteReference {
    note_id!: string;
    reference_type!: string;
    reference_id!: string;
    timestamp?: Date;
    metadata?: Record<string, any>;
    created_at!: Date;

    constructor(data: Partial<NoteReference>) {
        Object.assign(this, data);
    }
}

export class KnownEntity implements BaseEntity {
    entity_id!: string;
    type!: EntityType;
    label!: string;
    embedding?: number[];
    metadata?: Record<string, any>;
    created_at!: Date;
    updated_at!: Date;

    constructor(data: Partial<KnownEntity>) {
        Object.assign(this, data);
    }
} 