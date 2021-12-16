import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { AuthClient } from '@dfinity/auth-client';
//1.引入stoic
import {StoicIdentity} from "ic-stoic-identity";

const signInBtn = document.getElementById('signinBtn');
const signOutBtn = document.getElementById('signoutBtn');
const whoamiBtn = document.getElementById('whoamiBtn');
const hostUrlEl = document.getElementById('hostUrl');
const whoAmIResponseEl = document.getElementById('whoamiResponse');
const canisterIdEl = document.getElementById('canisterId');
const principalEl = document.getElementById('principal');
const idpUrlEl = document.getElementById('idpUrl');

let authClient;
let myIdentity;

const init = async () => {
  authClient = await AuthClient.create();
  console.log(authClient);
  principalEl.innerText = await authClient.getIdentity().getPrincipal();

  //2.重定向至stoic钱包登录界面
  signInBtn.onclick = async () => {
        StoicIdentity.load().then(async identity => {
          if (identity !== false) {
            //ID is a already connected wallet!
            console.log("ID is a already connected wallet!");
          } else {
            //No existing connection, lets make one!
            console.log("No existing connection, lets make one!");
            identity = await StoicIdentity.connect();
            
          }
          
          //Lets display the connected principal!
          console.log(identity.getPrincipal().toText());
          myIdentity = identity;
          
        });
  };

  signOutBtn.onclick = async () => {
    StoicIdentity.disconnect();
  };
};

init();

whoamiBtn.addEventListener('click', async () => {
  console.log('My Principal:',myIdentity.getPrincipal().toText());
  principalEl.innerText = myIdentity.getPrincipal().toText();
  const identity = myIdentity;

  //3. 定义容器的接口描述（容器编译完成后会自动生成）
  // We either have an Agent with an anonymous identity (not authenticated),
  // or already authenticated agent, or parsing the redirect from window.location.
  const idlFactory = ({ IDL }) =>
    IDL.Service({
      'whoami' : IDL.Func([], [IDL.Principal], ['query']),
      'get' : IDL.Func([], [IDL.Nat], ['query']),
      'inc' : IDL.Func([], [], []),
      'set' : IDL.Func([IDL.Nat], [], []),
      'get_token_properties' : IDL.Func([IDL.Nat64],[IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],['query'],),
      'claim' : IDL.Func([], [IDL.Nat64], []),
    });

  const canisterId = Principal.fromText(canisterIdEl.value);

  //4.实例化“Actor”，相当于实例化一个钱包对象，用它去与后端容器交互
  const actor = Actor.createActor(idlFactory, {
    agent: new HttpAgent({
      host: hostUrlEl.value,
      identity,
    }),
    canisterId,
  });

  whoAmIResponseEl.innerText = 'Loading...';

  // Similar to the sample project on dfx new:
  //4k2wq-cqaaa-aaaab-qac7q-cai: get inc set
  //qlfqk-fqaaa-aaaah-qakfq-cai: whoami
  //6jnqp-3qaaa-aaaah-qcbpq-cai: ic-Rush

//5.调用容器函数
  actor.claim().then(principal => {
  //actor.set(123).then(principal => {
  //actor.whoami().then(principal => {
    console.log('Return:',principal);
    whoAmIResponseEl.innerText = principal.toText();
    //whoAmIResponseEl.innerText = "111222";
  });
});
