import{S as B,i as z,s as H,c as W,e as $,a as o,b as j,d as u,u as X,g as Y,f as Z,t as m,h as g,j as C,o as K,k as x,l as Q,m as k,n as y,p as P,q as ee,r as E,v as R,w as U,x as F,y as M,z as N,A as q,I as te,B as ne,C as le,D as O,E as se,F as re,G as ae,H as V,J as A,K as oe,L as D,M as I}from"./index-b17b27f5.js";import{c as ce,N as ie}from"./Navbar-b091dd1d.js";function ue(c){let e,n,t;const l=c[2].default,r=W(l,c,c[1],null);return{c(){e=$("div"),n=$("div"),r&&r.c(),o(n,"class","portal-inner"),o(e,"class","portal-clone svelte-1tufk5f")},m(s,i){j(s,e,i),u(e,n),r&&r.m(n,null),c[3](n),t=!0},p(s,[i]){r&&r.p&&(!t||i&2)&&X(r,l,s,s[1],t?Z(l,s[1],i,null):Y(s[1]),null)},i(s){t||(m(r,s),t=!0)},o(s){g(r,s),t=!1},d(s){s&&C(e),r&&r.d(s),c[3](null)}}}function fe(c,e,n){let{$$slots:t={},$$scope:l}=e,r,s;K(()=>{s=document.createElement("div"),s.className="portal",document.body.appendChild(s),s.appendChild(r)}),x(()=>{document.body.removeChild(s)});function i(a){Q[a?"unshift":"push"](()=>{r=a,n(0,r)})}return c.$$set=a=>{"$$scope"in a&&n(1,l=a.$$scope)},[r,l,t,i]}class pe extends B{constructor(e){super(),z(this,e,fe,ue,H,{})}}function de(c){let e,n,t,l;return{c(){e=k("svg"),n=k("path"),t=k("circle"),l=k("path"),o(n,"stroke","none"),o(n,"d","M0 0h24v24H0z"),o(n,"fill","none"),o(t,"cx","8.5"),o(t,"cy","8.5"),o(t,"r","1"),o(t,"fill","currentColor"),o(l,"d","M4 7v3.859c0 .537 .213 1.052 .593 1.432l8.116 8.116a2.025 2.025 0 0 0 2.864 0l4.834 -4.834a2.025 2.025 0 0 0 0 -2.864l-8.117 -8.116a2.025 2.025 0 0 0 -1.431 -.593h-3.859a3 3 0 0 0 -3 3z"),o(e,"xmlns","http://www.w3.org/2000/svg"),o(e,"class","icon icon-tabler icon-tabler-tag"),o(e,"width","24"),o(e,"height","24"),o(e,"viewBox","0 0 24 24"),o(e,"stroke-width","2"),o(e,"stroke","currentColor"),o(e,"fill","none"),o(e,"stroke-linecap","round"),o(e,"stroke-linejoin","round")},m(r,s){j(r,e,s),u(e,n),u(e,t),u(e,l)},p:y,i:y,o:y,d(r){r&&C(e)}}}class _e extends B{constructor(e){super(),z(this,e,null,de,H,{})}}function me(c){let e,n,t,l,r;return{c(){e=k("svg"),n=k("path"),t=k("path"),l=k("path"),r=k("path"),o(n,"stroke","none"),o(n,"d","M0 0h24v24H0z"),o(n,"fill","none"),o(t,"d","M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5"),o(l,"d","M16 19h6"),o(r,"d","M19 16v6"),o(e,"xmlns","http://www.w3.org/2000/svg"),o(e,"class","icon icon-tabler icon-tabler-folder-plus"),o(e,"width","24"),o(e,"height","24"),o(e,"viewBox","0 0 24 24"),o(e,"stroke-width","2"),o(e,"stroke","currentColor"),o(e,"fill","none"),o(e,"stroke-linecap","round"),o(e,"stroke-linejoin","round")},m(s,i){j(s,e,i),u(e,n),u(e,t),u(e,l),u(e,r)},p:y,i:y,o:y,d(s){s&&C(e)}}}class he extends B{constructor(e){super(),z(this,e,null,me,H,{})}}function J(c){let e,n;return e=new pe({props:{$$slots:{default:[ve]},$$scope:{ctx:c}}}),{c(){M(e.$$.fragment)},m(t,l){N(e,t,l),n=!0},p(t,l){const r={};l&271&&(r.$$scope={dirty:l,ctx:t}),e.$set(r)},i(t){n||(m(e.$$.fragment,t),n=!0)},o(t){g(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function $e(c){let e,n;return e=new _e({}),{c(){M(e.$$.fragment)},m(t,l){N(e,t,l),n=!0},i(t){n||(m(e.$$.fragment,t),n=!0)},o(t){g(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function ge(c){let e,n;return e=new he({}),{c(){M(e.$$.fragment)},m(t,l){N(e,t,l),n=!0},i(t){n||(m(e.$$.fragment,t),n=!0)},o(t){g(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function ve(c){let e,n,t,l,r,s,i,a,f,h,_;function d(p){c[6](p)}let b={type:"text",placeholder:"Project name",error:c[1].name,$$slots:{default:[$e]},$$scope:{ctx:c}};return c[0].name!==void 0&&(b.value=c[0].name),r=new te({props:b}),Q.push(()=>ne(r,"value",d)),a=new le({props:{loading:c[3],text:"Create",$$slots:{default:[ge]},$$scope:{ctx:c}}}),a.$on("click",c[5]),{c(){e=$("div"),n=$("form"),t=$("span"),t.textContent="New project",l=P(),M(r.$$.fragment),i=P(),M(a.$$.fragment),o(t,"class","svelte-bk1fwi"),o(n,"class","modal-content svelte-bk1fwi"),o(e,"class","hidden svelte-bk1fwi"),O(e,"open",c[2])},m(p,v){j(p,e,v),u(e,n),u(n,t),u(n,l),N(r,n,null),u(n,i),N(a,n,null),f=!0,h||(_=[se(ce.call(null,e)),E(e,"click_outside",c[7])],h=!0)},p(p,v){const S={};v&2&&(S.error=p[1].name),v&256&&(S.$$scope={dirty:v,ctx:p}),!s&&v&1&&(s=!0,S.value=p[0].name,re(()=>s=!1)),r.$set(S);const w={};v&8&&(w.loading=p[3]),v&256&&(w.$$scope={dirty:v,ctx:p}),a.$set(w),(!f||v&4)&&O(e,"open",p[2])},i(p){f||(m(r.$$.fragment,p),m(a.$$.fragment,p),f=!0)},o(p){g(r.$$.fragment,p),g(a.$$.fragment,p),f=!1},d(p){p&&C(e),q(r),q(a),h=!1,ae(_)}}}function we(c){let e,n,t,l,r,s,i=c[2]&&J(c);return{c(){e=$("button"),e.innerHTML='<span class="svelte-bk1fwi">Create project +</span>',n=P(),i&&i.c(),t=ee(),o(e,"class","svelte-bk1fwi")},m(a,f){j(a,e,f),j(a,n,f),i&&i.m(a,f),j(a,t,f),l=!0,r||(s=E(e,"click",c[4]),r=!0)},p(a,[f]){a[2]?i?(i.p(a,f),f&4&&m(i,1)):(i=J(a),i.c(),m(i,1),i.m(t.parentNode,t)):i&&(R(),g(i,1,1,()=>{i=null}),U())},i(a){l||(m(i),l=!0)},o(a){g(i),l=!1},d(a){a&&(C(e),C(n),C(t)),i&&i.d(a),r=!1,s()}}}function ke(c,e,n){let t={name:""},l=F.make(t),r=!1,s=!1;async function i(){n(2,r=!0)}async function a(){n(3,s=!0);const _=await fetch("/api/projects",{method:"POST",body:JSON.stringify(t),headers:{"Content-Type":"application/json"}});if(!_.ok){n(1,l=F.apply(l,await _.json())),n(3,s=!1);return}const{project:d}=await _.json();V(`#/admin/project/${d.id}`),n(3,s=!1)}function f(_){c.$$.not_equal(t.name,_)&&(t.name=_,n(0,t))}return[t,l,r,s,i,a,f,()=>n(2,r=!1)]}class be extends B{constructor(e){super(),z(this,e,ke,we,H,{})}}function je(c){let e,n,t;return{c(){e=k("svg"),n=k("path"),t=k("path"),o(n,"stroke","none"),o(n,"d","M0 0h24v24H0z"),o(n,"fill","none"),o(t,"d","M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2"),o(e,"xmlns","http://www.w3.org/2000/svg"),o(e,"class","icon icon-tabler icon-tabler-folder"),o(e,"width","24"),o(e,"height","24"),o(e,"viewBox","0 0 24 24"),o(e,"stroke-width","2"),o(e,"stroke","currentColor"),o(e,"fill","none"),o(e,"stroke-linecap","round"),o(e,"stroke-linejoin","round")},m(l,r){j(l,e,r),u(e,n),u(e,t)},p:y,i:y,o:y,d(l){l&&C(e)}}}class Ce extends B{constructor(e){super(),z(this,e,null,je,H,{})}}function L(c,e,n){const t=c.slice();return t[2]=e[n],t}function G(c){let e,n,t,l,r=c[2].name+"",s,i,a,f=(c[2].features_count??0)+"",h,_,d,b,p,v;t=new Ce({});function S(){return c[1](c[2])}return{c(){e=$("button"),n=$("span"),M(t.$$.fragment),l=P(),s=D(r),i=P(),a=$("span"),h=D(f),_=D(" features"),d=P(),o(n,"class","svelte-e33twq"),o(a,"class","svelte-e33twq"),o(e,"class","project svelte-e33twq")},m(w,T){j(w,e,T),u(e,n),N(t,n,null),u(n,l),u(n,s),u(e,i),u(e,a),u(a,h),u(a,_),u(e,d),b=!0,p||(v=E(e,"click",S),p=!0)},p(w,T){c=w,(!b||T&1)&&r!==(r=c[2].name+"")&&I(s,r),(!b||T&1)&&f!==(f=(c[2].features_count??0)+"")&&I(h,f)},i(w){b||(m(t.$$.fragment,w),b=!0)},o(w){g(t.$$.fragment,w),b=!1},d(w){w&&C(e),q(t),p=!1,v()}}}function ye(c){let e,n,t=A(c[0]),l=[];for(let s=0;s<t.length;s+=1)l[s]=G(L(c,t,s));const r=s=>g(l[s],1,1,()=>{l[s]=null});return{c(){e=$("div");for(let s=0;s<l.length;s+=1)l[s].c();o(e,"class","projects svelte-e33twq")},m(s,i){j(s,e,i);for(let a=0;a<l.length;a+=1)l[a]&&l[a].m(e,null);n=!0},p(s,[i]){if(i&1){t=A(s[0]);let a;for(a=0;a<t.length;a+=1){const f=L(s,t,a);l[a]?(l[a].p(f,i),m(l[a],1)):(l[a]=G(f),l[a].c(),m(l[a],1),l[a].m(e,null))}for(R(),a=t.length;a<l.length;a+=1)r(a);U()}},i(s){if(!n){for(let i=0;i<t.length;i+=1)m(l[i]);n=!0}},o(s){l=l.filter(Boolean);for(let i=0;i<l.length;i+=1)g(l[i]);n=!1},d(s){s&&C(e),oe(l,s)}}}function Pe(c,e,n){let t=[];return K(async()=>{const r=await fetch("/api/projects"),{projects:s}=await r.json();n(0,t=s)}),[t,r=>V(`#/admin/project/${r.id}`)]}class Me extends B{constructor(e){super(),z(this,e,Pe,ye,H,{})}}function Ne(c){let e,n,t,l,r,s,i,a,f,h,_;return n=new ie({}),a=new be({}),h=new Me({}),{c(){e=$("div"),M(n.$$.fragment),t=P(),l=$("div"),r=$("header"),s=$("h1"),s.textContent="Projects",i=P(),M(a.$$.fragment),f=P(),M(h.$$.fragment),o(r,"class","page-header"),o(l,"class","page")},m(d,b){j(d,e,b),N(n,e,null),u(e,t),u(e,l),u(l,r),u(r,s),u(r,i),N(a,r,null),u(l,f),N(h,l,null),_=!0},p:y,i(d){_||(m(n.$$.fragment,d),m(a.$$.fragment,d),m(h.$$.fragment,d),_=!0)},o(d){g(n.$$.fragment,d),g(a.$$.fragment,d),g(h.$$.fragment,d),_=!1},d(d){d&&C(e),q(n),q(a),q(h)}}}class ze extends B{constructor(e){super(),z(this,e,null,Ne,H,{})}}export{ze as default};
