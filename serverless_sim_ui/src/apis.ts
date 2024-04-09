import axios from "axios"



class GetTopoRespExist {
    constructor(
        public topo:number[][],
    ){}
}

class GetTopoRespNotFound {
    constructor(
        public msg:string,
    ){}
}

class GetTopoResp{
    kernel: any
    private id: number=0
    
    exist():undefined| GetTopoRespExist{
        if(this.id==1){
            return this.kernel
        }
        return undefined
    }
    
    not_found():undefined| GetTopoRespNotFound{
        if(this.id==2){
            return this.kernel
        }
        return undefined
    }
    
}


class GetTopoReq {
    constructor(
        public env_id:string,
    ){}
}

export namespace apis {
    async function get_topo(req:GetTopoReq):Promise<GetTopoResp>{
        return await axios.post("/api/get_topo", req)
    }
}


