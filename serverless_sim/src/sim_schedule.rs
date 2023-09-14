use std::collections::{ BTreeSet, HashMap };

use enum_as_inner::EnumAsInner;

use crate::{
    fn_dag::{ FnContainer, FnContainerState, FnId },
    node::{ Node, NodeId },
    request::{ ReqId, Request },
    sim_env::SimEnv,
    sim_ef_faas_flow::FaasFlowScheduler,
};

pub trait Scheduler {
    fn schedule_some(&mut self, env: &SimEnv);
}

#[derive(EnumAsInner)]
pub enum SchedulerImpl {
    FaasFlowScheduler(FaasFlowScheduler),
}

#[derive(Clone, Debug)]
struct TransPath {
    // from_node_id: NodeId,
    // to_node_id: NodeId,
    /// recv req
    req_id: ReqId,
    /// recv fn
    fn_id: FnId,
}

struct NodeTrans {
    send_paths: Vec<TransPath>,
    recv_paths: Vec<TransPath>,
}

impl NodeTrans {
    fn path_cnt(&self) -> usize {
        self.send_paths.len() + self.recv_paths.len()
    }
}

type NodeTransMap = HashMap<(NodeId, NodeId), NodeTrans>;

impl SimEnv {
    pub fn schedule_reqfn_on_node(&self, req: &mut Request, fnid: FnId, nodeid: NodeId) {
        // schedule on node
        let new_fn_running = self.fn_new_fn_running_state(req, fnid);
        self.nodes
            .borrow_mut()
            [nodeid].fn_containers.get_mut(&fnid)
            .unwrap()
            .req_fn_state.insert(req.req_id, new_fn_running);
        req.fn_node.insert(fnid, nodeid);
    }
    fn schedule_one_req_fns(&self, req: &mut Request) {
        let dag_i = req.dag_i;
        let mut dag_walker = self.dag(dag_i).new_dag_walker();
        let mut schedule_able_fns = vec![];
        'next_fn: while let Some(fngi) = dag_walker.next(&*self.dag_inner(dag_i)) {
            let fnid = self.dag_inner(dag_i)[fngi];
            if req.fn_node.contains_key(&fnid) {
                //scheduled
                continue;
            }
            let parents = self.func(fnid).parent_fns(self);
            for p in &parents {
                if !req.done_fns.contains(p) {
                    continue 'next_fn;
                }
            }
            if
                self.fn_2_nodes.borrow().contains_key(&fnid) &&
                self.fn_2_nodes.borrow().get(&fnid).unwrap().len() > 0
            {
                // parents all done schedule able
                schedule_able_fns.push(fnid);
            }
        }
        for &fnid in &schedule_able_fns {
            let fn_2_nodes = self.fn_2_nodes.borrow();
            let nodes = fn_2_nodes.get(&fnid).unwrap();
            let mut best_node = None;
            for &n in nodes {
                let time = self.algo_predict_fn_on_node_work_time(req, fnid, n);
                if let Some((best_n, besttime)) = best_node.take() {
                    if time < besttime {
                        best_node = Some((n, time));
                    } else {
                        best_node = Some((best_n, besttime));
                    }
                } else {
                    best_node = Some((n, time));
                }
            }
            let node_to_run_req_fn = best_node.unwrap().0;
            self.schedule_reqfn_on_node(req, fnid, node_to_run_req_fn);
        }
    }
    pub fn try_put_fn(&self) {
        log::info!("try put fn");
        //针对所有请求，将请求可以放的的fn放到可以放的fn容器中，

        // 要求进入的fn容器需要离前驱fn所进入的容器所在节点尽可能近，

        // 能进入的前提是，
        //  1.前驱fn已经执行完了
        //  2.fn instance 存在
        for (_req_id, req) in self.requests.borrow_mut().iter_mut() {
            self.schedule_one_req_fns(req);
            // if let Some((fnid, _fn_g_i)) = req.fn_2_bind_node() {
            //     let env_fn_2_nodes = &self.fn_2_nodes.borrow();
            //     //对应请求还有未调度的fn
            //     let parents = self.func(fnid).parent_fns(self);

            //     let collect_parent_fns_nodes = || -> Option<Vec<(FnId, NodeId)>> {
            //         let mut parents_fns_nodes = vec![];
            //         for p in &parents {
            //             let p_node = req.fn_node.get(&p);
            //             if let Some(p_node) = p_node {
            //                 if !req.done_fns.contains(p) {
            //                     //前驱fn还没执行完
            //                     return None;
            //                 }
            //                 parents_fns_nodes.push((*p, *p_node));
            //             } else {
            //                 return None;
            //             }
            //         }
            //         Some(parents_fns_nodes)
            //     };

            //     let parent_fns_nodes = if let Some(parent_fns_nodes) = collect_parent_fns_nodes() {
            //         parent_fns_nodes
            //     } else {
            //         continue;
            //     };

            //     let fn_nodes = env_fn_2_nodes.get(&fnid);
            //     if fn_nodes.is_none() {
            //         //当前fn暂时不可调度
            //         // log::warn!("current fn{current_f} of req{req_id} has no node container");
            //         // break;
            //     } else {
            //         // 选出最优node用于执行
            //         let node_to_run_req_fn: NodeId = if self.fn_is_fn_dag_begin(req.dag_i, fnid) {
            //             // 若为dag 第一个f，采用负载均衡原则来选择位置
            //             let found_opt = self.algo_find_the_most_idle_node_for_fn(fnid);
            //             let found = if let Some(found) = found_opt {
            //                 found
            //             } else {
            //                 // 节点资源不够，先调度后面的请求
            //                 log::info!(
            //                     "failed to find node to run task, will be scheduled in later rounds"
            //                 );
            //                 continue;
            //             };
            //             if self.node(found).left_mem() < self.func(fnid).mem {
            //                 // 节点资源不够，先调度后面的请求
            //                 log::info!(
            //                     "node {} has no enough mem, will be scheduled in later rounds",
            //                     found
            //                 );
            //                 continue;
            //             }

            //             found
            //         } else {
            //             // 若为dag 其他f，与前置f所在node越近越好
            //             if let Some(node) =
            //                 self.algo_find_the_most_fast_node_for_req_fn(&parent_fns_nodes, |n| {
            //                     n.left_mem() > self.func(fnid).mem
            //                         && n.fn_containers.contains_key(&fnid)
            //                 })
            //             {
            //                 node
            //             } else {
            //                 log::info!("no node found for req {req_id} fn {fnid} to run currently");
            //                 continue;
            //             }
            //         };

            //         log::info!(
            //             "will put req{} fn{} instance on node{}, node mem from {} to {} / {}",
            //             req.req_id,
            //             fnid,
            //             node_to_run_req_fn,
            //             self.node(node_to_run_req_fn).mem,
            //             self.node(node_to_run_req_fn).mem + self.func(fnid).mem,
            //             self.node(node_to_run_req_fn).rsc_limit.mem
            //         );
            //         self.node_mut(node_to_run_req_fn).mem += self.func(fnid).mem;

            //         let new_fn_running = self.fn_new_fn_running_state(req, fnid);
            //         self.nodes.borrow_mut()[node_to_run_req_fn]
            //             .fn_containers
            //             .get_mut(&fnid)
            //             .unwrap()
            //             .req_fn_state
            //             .insert(req.req_id, new_fn_running);
            //         req.fn_node.insert(fnid, node_to_run_req_fn);
            //         let dag = self.dag_inner(req.dag_i);

            //         req.prepare_next_fn_2_bind_node(&*dag);
            //     }
            // }
        }
    }

    fn sim_transfer_btwn_nodes(&self, node_a: NodeId, node_b: NodeId, transmap: &mut NodeTransMap) {
        assert_ne!(node_a, node_b);

        // 两个node之间的数据传输
        let a2b = transmap.remove(&mut (node_a, node_b)).unwrap();
        let _b2a = transmap.remove(&mut (node_b, node_a)).unwrap();
        let total_bandwith = self.node_get_speed_btwn(node_a, node_b);
        let each_path_bandwith = total_bandwith / (a2b.path_cnt() as f32);

        let updata_trans = |from: NodeId, to: NodeId, t: &TransPath| {
            let mut env_nodes = self.nodes.borrow_mut();
            let mut container = env_nodes[to].fn_containers
                .get_mut(&t.fn_id)
                .unwrap_or_else(|| panic!("node {} has no fn container for fn {}", to, t.fn_id));
            container.this_frame_used = true;

            let (all, recved) = container.req_fn_state
                .get_mut(&t.req_id)
                .unwrap()
                .data_recv.get_mut(&from)
                .unwrap();
            if *all < *recved {
                // 该数据已经传输完毕
                log::info!(
                    "data from {from} to {to} for req{} fn{} has been transfered",
                    t.req_id,
                    t.fn_id
                );
            } else {
                *recved += each_path_bandwith;
            }
        };

        // a，b之间单个任务的传输速度，取决于a，b间的路径数
        for t in a2b.send_paths {
            // a2b
            updata_trans(node_a, node_b, &t);
        }

        for t in a2b.recv_paths {
            updata_trans(node_b, node_a, &t);
        }
    }

    fn sim_transfers(&self) {
        // 收集所有node向其他函数发送和接收的路径数，这样每个接收函数可以知道从远程node收到多少数据，
        // 模拟传输时，一个一个路径遍历过来，
        //   两边一定有一个网速更快，那么选择慢的；然后快的那边可以把带宽分给其他的传输路径
        //
        let mut node2node_trans: NodeTransMap = HashMap::new();
        for x in 0..self.nodes.borrow().len() {
            for y in 0..self.nodes.borrow().len() {
                if x != y {
                    node2node_trans.insert((x, y), NodeTrans {
                        send_paths: vec![],
                        recv_paths: vec![],
                    });
                }
            }
        }

        // go through all the fn task scheduled on node, and collect the transfer paths
        for node in self.nodes.borrow_mut().iter_mut() {
            let node_id = node.node_id();
            for (fnid, fn_container) in &mut node.fn_containers {
                for (req_id, fnrun) in &mut fn_container.req_fn_state {
                    for (send_node, (all, recved)) in &mut fnrun.data_recv {
                        // 数据还没接受完才需要传输
                        if *recved < *all {
                            if *send_node == node_id {
                                // data transfer on same node can be done immediately
                                *recved = *all + 0.001;
                            } else {
                                let path = TransPath {
                                    req_id: *req_id,
                                    fn_id: *fnid,
                                };
                                // log::info!("new one path: {path:?} to node {node_id}");
                                let send_2_recv = node2node_trans
                                    .get_mut(&(*send_node, node_id))
                                    .unwrap();
                                send_2_recv.send_paths.push(path.clone());

                                let recv_2_send = node2node_trans
                                    .get_mut(&(node_id, *send_node))
                                    .unwrap();
                                recv_2_send.recv_paths.push(path.clone());
                            }
                        }
                    }
                }
            }
        }
        // go through all the transfer paths, and simulate the transfer
        let nodes_cnt = self.nodes.borrow().len();
        for x in 0..nodes_cnt {
            for y in 0..nodes_cnt {
                if x > y {
                    // simu transfer between node x and y
                    self.sim_transfer_btwn_nodes(x, y, &mut node2node_trans);
                }
            }
        }
    }

    // return true means state move on
    fn sim_compute_container_starting(
        &self,
        fnid: FnId,
        fc: &mut FnContainer,
        cpu_for_one_task: f32
    ) {
        let container_cpu_used = cpu_for_one_task.min(self.func(fnid).cold_start_container_cpu_use);
        fc.set_cpu_use_rate(cpu_for_one_task, container_cpu_used);

        fc.starting_left_frame_move_on();
    }

    fn sim_compute_container_running(
        &self,
        fnid: FnId,
        container_node_cpu: &mut f32,
        fc: &mut FnContainer,
        cpu_for_one_task: f32,
        req_fns_2_run: &BTreeSet<(ReqId, FnId)>
    ) {
        let mut done_reqs = vec![];
        let mut calc_cnt = 0;

        // used to compute cpu use rate
        let mut container_alloced_cpu = 0.0;
        let mut container_used_cpu = 0.0;

        for (reqid, fn_running_state) in &mut fc.req_fn_state {
            if !req_fns_2_run.contains(&(fnid, *reqid)) {
                continue;
            }
            calc_cnt += 1;

            // calc process
            let used_cpu = cpu_for_one_task.min(fn_running_state.left_calc);
            fn_running_state.left_calc -= cpu_for_one_task;
            *container_node_cpu += used_cpu;

            // cpu suppose to use
            container_alloced_cpu += cpu_for_one_task;
            // cpu really used
            container_used_cpu += used_cpu;

            if fn_running_state.compute_done() {
                done_reqs.push(*reqid);
            }
        }

        //有计算，容器被使用
        if calc_cnt > 0 {
            fc.this_frame_used = true;
            // compute cpu use rate
            fc.set_cpu_use_rate(container_alloced_cpu, container_used_cpu);
        } else {
            fc.set_cpu_use_rate(1.0, 0.0);
        }

        fc.record_this_frame(self, done_reqs.len(), fc.req_fn_state.len());
        for reqid in done_reqs {
            fc.req_fn_state.remove(&reqid).unwrap();
            let mut req = self.request_mut(reqid);
            req.fn_done(self, fnid, self.current_frame());
            if req.is_done(self) {
                drop(req);
                self.on_request_done(reqid);
            }
        }
    }

    fn sim_compute_collect_compute_data(
        &self,
        n: &mut Node
    ) -> Option<(BTreeSet<(ReqId, FnId)>, usize, f32)> {
        let mut req_fns_2_run = BTreeSet::new();

        // collect run fn count, alloc cpu resource equally
        let starting_container_cnt = n.fn_containers
            .iter()
            .filter(|(_, fc)| {
                match fc.state() {
                    FnContainerState::Starting { .. } => true,
                    _ => false,
                }
            })
            .count();

        for (&fnid, fc) in &n.fn_containers {
            if let FnContainerState::Running { .. } = fc.state() {
                for (&req_id, fn_running_state) in &fc.req_fn_state {
                    if fn_running_state.data_recv_done() && n.left_mem() > self.func(fnid).mem {
                        n.mem += self.func(fnid).mem;
                        req_fns_2_run.insert((fnid, req_id));
                    }
                }
            }
        }

        // n.mem = used_mem;
        if req_fns_2_run.len() == 0 && starting_container_cnt == 0 {
            None
        } else {
            // 计算任务数，每个任务平分计算量
            let each_fn_cpu =
                n.rsc_limit.cpu / ((req_fns_2_run.len() + starting_container_cnt) as f32);
            n.frame_run_count = req_fns_2_run.len() + starting_container_cnt;
            Some((req_fns_2_run, starting_container_cnt, each_fn_cpu))
        }
    }

    fn sim_computes(&self) {
        for n in self.nodes.borrow_mut().iter_mut() {
            if
                let Some((req_fns_2_run, _starting_container_cnt, cpu_for_one_task)) =
                    self.sim_compute_collect_compute_data(n)
            {
                for (fnid, fc) in &mut n.fn_containers {
                    match fc.state_mut() {
                        FnContainerState::Starting { .. } => {
                            self.sim_compute_container_starting(*fnid, fc, cpu_for_one_task);
                            if let FnContainerState::Running = fc.state() {
                                // starting -> running
                                n.mem -= self.func(*fnid).cold_start_container_mem_use;
                                n.mem += self.func(*fnid).container_mem();
                            }
                        }
                        FnContainerState::Running =>
                            self.sim_compute_container_running(
                                *fnid,
                                &mut n.cpu,
                                fc,
                                cpu_for_one_task,
                                &req_fns_2_run
                            ),
                    }
                }
            } else {
                for (fnid, fc) in &mut n.fn_containers {
                    match fc.state_mut() {
                        FnContainerState::Starting { .. } => {
                            panic!("should not be starting");
                        }
                        FnContainerState::Running =>
                            self.sim_compute_container_running(
                                *fnid,
                                &mut n.cpu,
                                fc,
                                0.0,
                                &BTreeSet::new()
                            ),
                    }
                }
            }
        }
    }

    pub fn sim_run(&self) {
        log::info!("sim run");

        self.sim_transfers();
        self.sim_computes();
    }
    pub fn schedule_fn(&self) {
        self.try_put_fn();
        self.sim_run();
    }
}
