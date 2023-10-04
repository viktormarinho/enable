import{S,i as B,s as T,I as G,u as O,v as F,B as H,f as g,p as v,g as h,b as y,c as k,q as w,d,l as J,y as z,t as m,k as _,e as N,r as C,L as M,M as K,n as D,C as A,j as Q,m as R,D as U,E as V,P as W,F as X,G as Y}from"./index-44eb64d1.js";import{C as Z,D as ee,B as te}from"./BackToProjects-becffead.js";import{N as ne}from"./Portal-39177507.js";import{T as re}from"./Tag-519bfd64.js";function se(c){let e,t;return e=new re({}),{c(){v(e.$$.fragment)},m(l,f){w(e,l,f),t=!0},i(l){t||(m(e.$$.fragment,l),t=!0)},o(l){_(e.$$.fragment,l),t=!1},d(l){C(e,l)}}}function ae(c){let e;return{c(){e=g("span"),e.textContent="+"},m(t,l){k(t,e,l)},p:D,d(t){t&&N(e)}}}function le(c){let e,t,l,f,s,r,u,n;function o(i){c[4](i)}let a={maxLength:"35",error:c[1],placeholder:"Credential name",$$slots:{default:[se]},$$scope:{ctx:c}};return c[0]!==void 0&&(a.value=c[0]),t=new G({props:a}),O.push(()=>F(t,"value",o)),s=new H({props:{text:"Create",loading:c[2],$$slots:{default:[ae]},$$scope:{ctx:c}}}),{c(){e=g("form"),v(t.$$.fragment),f=h(),v(s.$$.fragment),y(e,"class","svelte-3stvjt")},m(i,p){k(i,e,p),w(t,e,null),d(e,f),w(s,e,null),r=!0,u||(n=J(e,"submit",c[3]),u=!0)},p(i,[p]){const j={};p&2&&(j.error=i[1]),p&128&&(j.$$scope={dirty:p,ctx:i}),!l&&p&1&&(l=!0,j.value=i[0],z(()=>l=!1)),t.$set(j);const b={};p&4&&(b.loading=i[2]),p&128&&(b.$$scope={dirty:p,ctx:i}),s.$set(b)},i(i){r||(m(t.$$.fragment,i),m(s.$$.fragment,i),r=!0)},o(i){_(t.$$.fragment,i),_(s.$$.fragment,i),r=!1},d(i){i&&N(e),C(t),C(s),u=!1,n()}}}function oe(c,e,t){const{addNotification:l}=M(),f=K();let s="",r="",u=!1;async function n(a){if(a.preventDefault(),t(1,r=""),t(2,u=!0),s.length<3){t(1,r="Please pick a name with at least 3 characters"),t(2,u=!1);return}(await fetch("/api/credentials",{method:"POST",body:JSON.stringify({name:s}),headers:{"Content-Type":"application/json"}})).ok?(l({type:"success",position:"top-right",removeAfter:3e3,text:"Credential created with success.",id:"credential-created"}),f("created"),t(0,s="")):l({type:"error",position:"top-right",removeAfter:3e3,text:"Could not create credential.",id:"credential-created-error"}),t(2,u=!1)}function o(a){s=a,t(0,s)}return[s,r,u,n,o]}class ie extends S{constructor(e){super(),B(this,e,oe,le,T,{})}}function L(c,e,t){const l=c.slice();return l[5]=e[t],l}function I(c){let e,t,l,f,s=c[5].name+"",r,u,n,o,a,i,p,j,b;t=new W({}),a=new Z({props:{text:c[5].token}});function q(){return c[3](c[5])}return p=new ee({props:{label:`the credential ${c[5].name}`}}),p.$on("confirm",q),{c(){e=g("li"),v(t.$$.fragment),l=h(),f=g("span"),r=X(s),u=h(),n=g("span"),n.textContent="••••••••••••••••••••••••••••••••••••••••••",o=h(),v(a.$$.fragment),i=h(),v(p.$$.fragment),j=h(),y(f,"class","name svelte-j2gxj4"),y(n,"class","pass svelte-j2gxj4"),y(e,"class","svelte-j2gxj4")},m($,x){k($,e,x),w(t,e,null),d(e,l),d(e,f),d(f,r),d(e,u),d(e,n),d(e,o),w(a,e,null),d(e,i),w(p,e,null),d(e,j),b=!0},p($,x){c=$,(!b||x&1)&&s!==(s=c[5].name+"")&&Y(r,s);const E={};x&1&&(E.text=c[5].token),a.$set(E);const P={};x&1&&(P.label=`the credential ${c[5].name}`),p.$set(P)},i($){b||(m(t.$$.fragment,$),m(a.$$.fragment,$),m(p.$$.fragment,$),b=!0)},o($){_(t.$$.fragment,$),_(a.$$.fragment,$),_(p.$$.fragment,$),b=!1},d($){$&&N(e),C(t),C(a),C(p)}}}function ce(c){let e,t,l,f,s=A(c[0]),r=[];for(let n=0;n<s.length;n+=1)r[n]=I(L(c,s,n));const u=n=>_(r[n],1,1,()=>{r[n]=null});return l=new ie({}),l.$on("created",c[1]),{c(){e=g("ul");for(let n=0;n<r.length;n+=1)r[n].c();t=h(),v(l.$$.fragment),y(e,"class","svelte-j2gxj4")},m(n,o){k(n,e,o);for(let a=0;a<r.length;a+=1)r[a]&&r[a].m(e,null);k(n,t,o),w(l,n,o),f=!0},p(n,[o]){if(o&5){s=A(n[0]);let a;for(a=0;a<s.length;a+=1){const i=L(n,s,a);r[a]?(r[a].p(i,o),m(r[a],1)):(r[a]=I(i),r[a].c(),m(r[a],1),r[a].m(e,null))}for(Q(),a=s.length;a<r.length;a+=1)u(a);R()}},i(n){if(!f){for(let o=0;o<s.length;o+=1)m(r[o]);m(l.$$.fragment,n),f=!0}},o(n){r=r.filter(Boolean);for(let o=0;o<r.length;o+=1)_(r[o]);_(l.$$.fragment,n),f=!1},d(n){n&&(N(e),N(t)),U(r,n),C(l,n)}}}function fe(c,e,t){const{addNotification:l}=M();let f=[];const s=async()=>{const n=await fetch("/api/credentials");t(0,f=await n.json())},r=async n=>{(await fetch(`/api/credentials/${n}`,{method:"DELETE"})).ok?(l({type:"success",position:"top-right",removeAfter:3e3,text:"Credential removed with success.",id:"credential-removed"}),s()):l({type:"error",position:"top-right",removeAfter:3e3,text:"Could not remove credential.",id:"credential-removed"})};return V(s),[f,s,r,n=>r(n.id)]}class ue extends S{constructor(e){super(),B(this,e,fe,ce,T,{})}}function pe(c){let e,t,l,f,s,r,u,n;return u=new ue({}),{c(){e=g("div"),t=g("section"),l=g("h2"),l.textContent="Credentials",f=h(),s=g("p"),s.textContent=`Generate and delete credentials for external access, such as integrating
      the enable API with your applications`,r=h(),v(u.$$.fragment),y(s,"class","svelte-ciyjsr"),y(t,"class","svelte-ciyjsr")},m(o,a){k(o,e,a),d(e,t),d(t,l),d(t,f),d(t,s),d(t,r),w(u,t,null),n=!0},p:D,i(o){n||(m(u.$$.fragment,o),n=!0)},o(o){_(u.$$.fragment,o),n=!1},d(o){o&&N(e),C(u)}}}class de extends S{constructor(e){super(),B(this,e,null,pe,T,{})}}function me(c){let e,t,l,f,s,r,u,n,o,a;return t=new ne({}),s=new te({}),o=new de({}),{c(){e=g("div"),v(t.$$.fragment),l=h(),f=g("div"),v(s.$$.fragment),r=h(),u=g("header"),u.innerHTML="<h1>Settings</h1>",n=h(),v(o.$$.fragment),y(u,"class","page-header"),y(f,"class","page")},m(i,p){k(i,e,p),w(t,e,null),d(e,l),d(e,f),w(s,f,null),d(f,r),d(f,u),d(f,n),w(o,f,null),a=!0},p:D,i(i){a||(m(t.$$.fragment,i),m(s.$$.fragment,i),m(o.$$.fragment,i),a=!0)},o(i){_(t.$$.fragment,i),_(s.$$.fragment,i),_(o.$$.fragment,i),a=!1},d(i){i&&N(e),C(t),C(s),C(o)}}}class ve extends S{constructor(e){super(),B(this,e,null,me,T,{})}}export{ve as default};
