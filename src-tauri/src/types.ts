export interface CurrentProject{
    version: number
    path: string;
}

export interface NoteNode {
    name:string;
    path:string;
    isDir: boolean;
    children: NoteNode[];
}