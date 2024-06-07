class x {
  //不能使用自带的构造方法
  //can't use the constructor method below
  constructor(e) {
    this.name = e;
  }
  transform;
  static create(e) {
    return Deno.core.ops.op_createGameObject(e);
  }
  addComponent(e) {
    let o = e.constructor;
    Deno.core.ops.op_addComponent(this.name, e, o.typeName);
  }
  getComponent(e) {
    return Deno.core.ops.op_getComponent(this.name, e.typeName);
  }
  getComponentById(e, o) {
    return Deno.core.ops.op_getComponentById(e, o);
  }
}
function D(t, e) {
  return Deno.core.ops.op_getComponentById(t, e);
}
globalThis.__GAMEOBJECT__ = new x("");
class d {
}
class w extends d {
  constructor(e) {
    super(), this.key = e;
  }
}
class g extends d {
  constructor(e, o) {
    super(), this.button = e, this.position = o;
  }
}
globalThis.__POST_MESSAGE__ = (t, e, ...o) => {
  switch (t) {
    case "keyboard": {
      let s = e;
      h.emit(t + s.state, new w(e.key));
      break;
    }
    case "mouse": {
      let s = e;
      h.emit(t + s.state, new g(e.button, e.position.toVec()));
      break;
    }
    case "mouse_move": {
      let s = e;
      h.emit(t + s.state, new g(e.button, e.position.toVec()));
      break;
    }
    case "ui": {
      print(`type:${t},data:${JSON.stringify(o)}`);
      let s = o[0][0].JsBind;
      D(s.objId, s.compId)[s.funcName]();
      break;
    }
    default:
      print(`type:${t},data:${JSON.stringify(e)}`);
  }
};
class f {
  constructor(e, o, s) {
    this.x = e, this.y = o, this.z = s;
  }
  into() {
    return [this.x, this.y, this.z];
  }
}
Object.defineProperty(Array.prototype, "x", {
  get: function() {
    return this[0];
  },
  set: function(t) {
    this[0] = t;
  }
});
Object.defineProperty(Array.prototype, "y", {
  get: function() {
    return this[1];
  },
  set: function(t) {
    this[1] = t;
  }
});
Object.defineProperty(Array.prototype, "z", {
  get: function() {
    return this[2];
  },
  set: function(t) {
    this[2] = t;
  }
});
Array.prototype.toVec = function() {
  return new f(this[0], this[1], this[2]);
};
const y = class {
  setTexture(e) {
  }
};
globalThis.__Material__ = new y();
const S = class {
};
globalThis.__Texture__ = new S();
const C = class {
  play() {
    return Deno.core.ops.opPlayAudio(this);
  }
  pause() {
    return Deno.core.ops.opPauseAudio(this);
  }
};
globalThis.__Audio__ = new C();
const N = class {
};
globalThis.__AudioClip__ = new N();
const k = class {
};
globalThis.__Mesh__ = new k();
const G = class {
};
globalThis.__Scene__ = new G();
class i {
  static typeName = "Component";
  name;
  parent;
  _node;
  get node() {
    return this._node == null && (this._node = this.getGameObject(this.parent)), this._node;
  }
  onStart() {
  }
  onUpdate(e) {
  }
  getComponent(e) {
    return Deno.core.ops.op_getComponent(this.parent, e.typeName);
  }
  getGameObject(e) {
    return Deno.core.ops.op_getGameObject(this, e);
  }
  getUiNode(e) {
    return Deno.core.ops.opGetUiNode(e);
  }
  toString() {
    return "Component {}";
  }
}
globalThis.__Component__ = new i();
globalThis.__Components__ = [];
class b {
  constructor(e, o, s, n) {
    this.func = e, this.now = o, this.time = s, this.once = n, this.now += s;
  }
  addNextTime() {
    this.now += this.time;
  }
}
class M {
  callbacks = [];
  addCallback(e) {
    return this.callbacks.push(e) - 1;
  }
  checkCall(e) {
    for (const o in this.callbacks) {
      let s = this.callbacks[o];
      s?.now < e && (s.func(), s.once ? this.callbacks[o] = void 0 : s.addNextTime());
    }
    this.callbacks = this.callbacks.filter((o) => o);
  }
}
const _ = new M();
globalThis.__Update__ = () => {
  _.checkCall(Date.now());
};
globalThis.setTimeout = (t, e) => {
  let o = new b(t, Date.now(), e, !0);
  _.addCallback(o);
};
globalThis.setInterval = (t, e) => {
  let o = new b(t, Date.now(), e, !1);
  _.addCallback(o);
};
const c = class extends i {
  setMaterial(e) {
    Deno.core.ops.opSetMaterial(this, e);
  }
};
c.typeName = "MaterialRender", globalThis.__MaterialRender__ = new c(), globalThis["##MaterialRender##"] = () => new c(), globalThis.__MaterialRender__ = new c();
const l = class extends i {
  setMesh(e) {
    Deno.core.ops.opSetMesh(this, e);
  }
};
l.typeName = "MeshRender", globalThis.__MeshRender__ = new l(), globalThis["##MeshRender##"] = () => new l(), globalThis.__MeshRender__ = new l();
const p = class extends i {
  get position() {
    return Deno.core.ops.opGetPosition(this).toVec();
  }
  setPosition(e) {
    return Deno.core.ops.opSetPosition(this, e.into());
  }
  get rotation() {
    return Deno.core.ops.opGetRotation(this).toVec();
  }
  setRotation(e) {
    return Deno.core.ops.opSetRotation(this, e.into());
  }
  get scale() {
    return Deno.core.ops.opGetScale(this).toVec();
  }
  setScale(e) {
    return Deno.core.ops.opSetScale(this, e.into());
  }
  translate(e) {
    return Deno.core.ops.opTranslate(this, e.into());
  }
  toString() {
    return `Transform { position:${JSON.stringify(this.position)}, rotation:${JSON.stringify(this.rotation)}, scale:${JSON.stringify(this.scale)}}`;
  }
};
p.typeName = "Transform", globalThis.__Transform__ = new p(), globalThis["##Transform##"] = () => new p(), globalThis.__Transform__ = new p();
const T = class {
  get active() {
    return Deno.core.ops.opGetWidgetActive(this);
  }
  set active(e) {
    Deno.core.ops.opSetWidgetActive(this, e);
  }
  get width() {
    return Deno.core.ops.opGetWidgetWidth(this);
  }
  set width(e) {
    Deno.core.ops.opSetWidgetWidth(this, e);
  }
  get height() {
    return Deno.core.ops.opGetWidgetHeight(this);
  }
  set height(e) {
    Deno.core.ops.opSetWidgetHeight(this, e);
  }
};
globalThis.__Widget__ = new T();
let a = T;
const A = class extends a {
  static create() {
    return Deno.core.ops.opCreateButton();
  }
  get text() {
    return Deno.core.ops.opGetButtonText(this);
  }
  set text(e) {
    Deno.core.ops.opSetButtonText(this, e);
  }
};
globalThis.__Button__ = new A();
const B = class extends a {
};
globalThis.__Grid__ = new B();
const O = class extends a {
};
globalThis.__Image__ = new O();
const P = class extends a {
  static create() {
    return Deno.core.ops.opCreateLabel();
  }
  get text() {
    return Deno.core.ops.opGetLabelText(this);
  }
  set text(e) {
    Deno.core.ops.opSetLabelText(this, e);
  }
};
globalThis.__Label__ = new P();
const R = class extends a {
  get orientation() {
    return Deno.core.ops.opGetPanelOrientation(this);
  }
  set orientation(e) {
    Deno.core.ops.opSetPanelOrientation(this, e);
  }
  get spacing() {
    return Deno.core.ops.opGetPanelSpacing(this);
  }
  set spacing(e) {
    Deno.core.ops.opSetPanelSpacing(this, e);
  }
};
globalThis.__Panel__ = new R();
const j = class extends a {
  static create() {
    return Deno.core.ops.opCreateTextBox();
  }
  get text() {
    return Deno.core.ops.opGetTextBoxText(this);
  }
  set text(e) {
    Deno.core.ops.opSetTextBoxText(this, e);
  }
  get placeholder() {
    return Deno.core.ops.opGetTextBoxHintText(this);
  }
  set placeholder(e) {
    Deno.core.ops.opSetTextBoxHintText(this, e);
  }
};
globalThis.__TextBox__ = new j();
const W = class {
  addChild(e) {
    Deno.core.ops.opAddChild(e);
  }
};
globalThis.__Canvas__ = new W();
const v = class {
  play(e) {
    return Deno.core.ops.opPlayAudioClip(e);
  }
};
globalThis.__AudioEngine__ = new v();
class I {
  /** 函数 function */
  func;
  /** 上下文 context */
  context;
  constructor(e, o) {
    this.func = e, this.context = o;
  }
}
class J {
  _messageTable;
  constructor() {
    this._messageTable = /* @__PURE__ */ new Map();
  }
  /**
   * 开始监听项 
   * start listening item
   * @param eventType 监听类型 listen type
   * @param handler 监听函数 listen function 
   * @param context 监听上下文 listen context
   */
  on(e, o, s) {
    let n = this._messageTable.get(e);
    n || (n = [], this._messageTable.set(e, n)), this.has(e, o) || n.push(new I(o, s));
  }
  /**
   * 移除监听项
   * remove listening item
   * @param eventType 事件类型 event type
   * @param handler 事件函数 event function 
   */
  off(e, o) {
    let s = this._messageTable.get(e);
    if (s) {
      let n = s.findIndex((m) => m.func == o);
      n != -1 && s.splice(n, 1);
    }
  }
  /**
   * 触发该事件
   * trigger this event 
   * @param eventType 事件类型 event type
   * @param data 事件数据 event data
   */
  emit(e, ...o) {
    let s = this._messageTable.get(e);
    if (s)
      for (let n of s)
        n.func.call(n.context, ...o);
  }
  /**
   * 判断是否存在该类型的观察者
   * judge if the viewer of this type exist  
   * @param eventType 事件类型 event type 
   * @param handler 事件函数 event function 
   */
  has(e, o) {
    let s = this._messageTable.get(e);
    return s ? s.some((n) => n.func === o) : !1;
  }
}
class L extends J {
}
const h = new L(), E = class {
  loadScene(e) {
    if (typeof e == "string")
      return Deno.core.ops.opLoadSceneFromName(e);
    if (typeof e == "object")
      return Deno.core.ops.opLoadScene(e), e;
  }
  preloadScene(e) {
    return Deno.core.ops.opPreloadScene(e);
  }
};
globalThis.__SceneManager__ = new E();
function $(t) {
  t == null ? Deno.core.print(`null
`) : t.constructor == String ? Deno.core.print(t + `
`) : t instanceof i ? Deno.core.print(t.toString() + `
`) : Deno.core.print(JSON.stringify(t) + `
`);
}
globalThis.print = $;
const r = class r extends i {
  onStart() {
  }
  onUpdate(e) {
  }
};
r.typeName = "Example", globalThis.__Example__ = new r(), globalThis["##Example##"] = () => new r(), globalThis.__Components__.push({ name: "Example", script: "example.ts" });
let u = r;
