<template>
  <div class="network-topology">
    <h1>网络拓扑图</h1>
    <div
        class="topology-container"
        @mousedown="startDragging"
        @mousemove="dragging"
        @mouseup="stopDragging"
        @mouseleave="stopDragging"
    >
      <div
          v-for="(node, index) in nodes"
          :key="index"
          :style="{ top: node.y + 'px', left: node.x + 'px', transform: 'translate(-50%, -50%)', zIndex: node.zIndex }"
          class="node"
          ref="nodes"
          @mousedown="startNodeDragging(index)"
          @mouseup="stopNodeDragging"
      >
        {{ node.id }}
        <button @click="removeNode(index)" class="remove-node-btn">删除</button>
      </div>
    </div>
    <svg class="connection-lines">
      <line
          v-for="(link, key) in links"
          :key="key"
          :x1="nodes[link.source[0]].x"
          :y1="nodes[link.source[0]].y"
          :x2="nodes[link.source[1]].x"
          :y2="nodes[link.source[1]].y"

      />
      <text
          v-for="(link, key) in links"
          :key="'text_' + key"
          :x="(nodes[link.source[0]].x + nodes[link.target[0]].x) / 2"
          :y="(nodes[link.source[0]].y + nodes[link.target[0]].y) / 2"
          dominant-baseline="middle"
          text-anchor="middle"
          fill="black"
          font-size="14"
      >
        {{ link.bandwidth }}
        <foreignObject :x="((nodes[link.source[0]].x + nodes[link.target[0]].x) / 2) - 20" :y="((nodes[link.source[0]].y + nodes[link.target[0]].y) / 2) - 20" width="40" height="40">
          <input type="text" v-model="link.bandwidth" style="width: 40px; height: 20px; font-size: 12px; padding: 2px;" />
        </foreignObject>
      </text>
    </svg>
    <button @click="addNode" class="add-node-btn">添加节点</button>
  </div>
</template>

<script>
import { UINode } from "@/network_topo";
import { UILink } from "@/network_topo";

export default {
  data() {
    return {
      nodes: [],
      links: new Map(),
      draggingNode: null,
      offset: { x: 0, y: 0 },
    };
  },
  methods: {
    startDragging(event) {
      this.offset.x = event.pageX;
      this.offset.y = event.pageY;
    },
    dragging(event) {
      if (this.draggingNode !== null) {
        const newX = event.pageX - this.offset.x + this.nodes[this.draggingNode].x;
        const newY = event.pageY - this.offset.y + this.nodes[this.draggingNode].y;
        this.nodes[this.draggingNode].x = newX;
        this.nodes[this.draggingNode].y = newY;
        this.offset.x = event.pageX;
        this.offset.y = event.pageY;
      }
    },
    stopDragging() {
      this.draggingNode = null;
    },
    startNodeDragging(index) {
      this.draggingNode = index;
      this.nodes[index].zIndex = 1;
    },
    stopNodeDragging() {
      if (this.draggingNode !== null) {
        this.nodes[this.draggingNode].zIndex = 0;
      }
      this.draggingNode = null;
    },
    removeNode(index) {
      this.nodes.splice(index, 1);
      this.links.forEach((link) => {
        if (link.source[0] === index || link.target[0] === index) {
          this.links.delete(link);
        }
      });
    },
    addNode() {
      const newNode = new UINode(Math.random() * 500, Math.random() * 500, 0, this.nodes.length);
      this.nodes.push(newNode);
      this.connectNodes(this.nodes.length - 1);
    },
    connectNodes(nodeIndex) {
      for (let i = 0; i < this.nodes.length; i++) {
        if (i !== nodeIndex) {
          const key = Math.min(nodeIndex, i) + '_' + Math.max(nodeIndex, i);
          if (!this.links.has(key)) {
            const newLink = new UILink([nodeIndex, i], 0, 0, 0);
            this.$set(this.links, key, newLink);
          }
        }
      }
    },
  },
};
</script>

<style>
.network-topology {
  padding: 20px;
}
.topology-container {
  position: relative;
  width: 600px;
  height: 400px;
  border: 1px solid #ccc;
}
.node {
  position: absolute;
  width: 80px;
  height: 80px;
  background-color: #2196F3;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  font-size: 16px;
  cursor: move;
}
.connection-lines {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}
.remove-node-btn {
  position: absolute;
  bottom: -20px;
  left: 50%;
  transform: translateX(-50%);
}
.add-node-btn {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
}
</style>
