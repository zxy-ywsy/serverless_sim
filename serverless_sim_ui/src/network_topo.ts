export class UINode{
    constructor(
        public x:number,
        public y:number,
        public zIndex:number,
        public id:number) {
    }
}

export class UILink{
    constructor(
        public source: { x: number, y: number },
        public target: { x: number, y: number },
        public bandwidth: number,
        public color: string
    ) {
    }
}

export class Topo{
    nodes: UINode[];
    links: UILink[];
}