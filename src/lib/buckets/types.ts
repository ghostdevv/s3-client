export interface Bucket {
    id: number;
    name: string;
    bucket: string;
    endpoint: string;
    region: string;
    key_id: string;
    key: string;
}

export interface FileMeta {
    size: number;
}
