import { BaseEntity } from './types';

export class User implements BaseEntity {
    id!: string;
    email!: string;
    name?: string;
    encrypted_password!: string;
    created_at!: Date;
    updated_at!: Date;

    constructor(data: Partial<User>) {
        Object.assign(this, data);
    }
}

export class OAuthAccount implements BaseEntity {
    id!: string;
    user_id!: string;
    provider!: string;
    provider_user_id!: string;
    access_token!: string;
    refresh_token?: string;
    expires_at?: Date;
    created_at!: Date;
    updated_at!: Date;

    constructor(data: Partial<OAuthAccount>) {
        Object.assign(this, data);
    }
}

export class Session {
    id!: string;
    user_id!: string;
    token!: string;
    expires_at!: Date;
    created_at!: Date;

    constructor(data: Partial<Session>) {
        Object.assign(this, data);
    }
} 