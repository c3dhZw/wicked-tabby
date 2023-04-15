function str() {
  return ('00000000000000000' + (Math.random() * 0xffffffffffffffff).toString(16)).slice(-16);
}

function uuid() {
  var a = str();
  var b = str();

  return a.slice(0, 8) + '-' + a.slice(8, 12) + '-4' + a.slice(13) + '-a' + b.slice(1, 4) + '-' + b.slice(4);
}

async function submit(e){
  if(e.preventDefault)
  e.preventDefault();

  var form = document.getElementById('poggers');
  var url = document.getElementById('url').value;
  var expire_date = document.getElementById('date').valueAsNumber;
  var can_expire = document.getElementById('expire').checked;

  const response = await fetch("create", {
    method: "POST",
    mode: "cors",
    cache: "no-cache",
    credentials: "same-origin",
    headers: {
      "Content-Type": "application/json",
    },
    redirect: "follow",
    referrerPolicy: "no-referrer",
    body: url+"|"+expire_date+"|"+can_expire+"|"+user_id,
  });

  let poggers = await response.json();

  if(poggers.code == 420){
    notify("failed to create url!");
  }

  if(poggers.code == 200){
    notify("new shortened link has been created!");
    existing_tab();
  }
}

function create_tab(){
  let tc = document.getElementById('tab_create').className="selected";
  document.getElementById('tab_existing').className="selectnt";

  document.getElementById('creation').style.display="";
  document.getElementById('existing').style.display="none";
}

function existing_tab(){
  document.getElementById('tab_create').className="selectnt";
  document.getElementById('tab_existing').className="selected";

  document.getElementById('creation').style.display="none";
  document.getElementById('existing').style.display="";

  refresh_existing();
}

let month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
"Jul", "Aug", "Sept", "Oct", "Nov", "Dec"
];

async function refresh_existing(){
  let table = document.getElementById('existing_tab');
  
  const response = await fetch("get_existing", {
    method: "POST",
    mode: "cors",
    cache: "no-cache",
    credentials: "same-origin",
    headers: {
      "Content-Type": "application/json",
    },
    redirect: "follow",
    referrerPolicy: "no-referrer",
    body: user_id,
  });

  let poggers = await response.json();

  if(poggers.code == 420){
    notify("failed to get existing!");
  }

  if(poggers.code == 200){
    table.innerHTML="";
    {
      let awa = document.createElement("tr");
      let nameh = document.createElement("th");
      nameh.innerText = "url";
      awa.appendChild(nameh);

      let clickiesh = document.createElement("th");
      clickiesh.innerText = "clickies";
      awa.appendChild(clickiesh);

      let expireh = document.createElement("th");
      expireh.innerText = "expires";
      awa.appendChild(expireh);

      let buttonh = document.createElement("th");
      buttonh.innerText = "shorten";
      awa.appendChild(buttonh);
      table.appendChild(awa);
    }

    poggers.data.forEach(a => {
      let awa = document.createElement("tr");
      
      let name = document.createElement("td");
      name.innerText = a.url;
      awa.appendChild(name);

      let clickies = document.createElement("td");
      clickies.innerText = a.clickies;
      awa.appendChild(clickies);

      let expire = document.createElement("td");

      if(a.expire_time){
        var date = new Date(a.expire_time);
        expire.innerText = date.getDay()+"-"+month[date.getMonth()-1]+-+date.getFullYear();
        awa.appendChild(expire);
      }else{
        expire.innerText = "never >:3";
        awa.appendChild(expire);
      }

      let button = document.createElement("td");
      button.innerHTML = "<a href=/"+a.id+"><button>🔗</button></a>";
      awa.appendChild(button);

      table.appendChild(awa);
    });


  }

  console.log(poggers);
}

function onLoad() {
  window.user_id = localStorage.getItem('id');

  console.log(window.user_id);

  if (window.user_id === null || window.user_id === '') {
    window.user_id = uuid();

    localStorage.setItem('id', window.user_id);
    notify('Generated new ID');
  }

  var form = document.getElementById('poggers');
  if (form.attachEvent) {
    form.attachEvent("submit", submit);
  } else {
    form.addEventListener("submit", submit);
  }

  var te = document.getElementById('tab_existing');
  if (te.attachEvent) {
    te.attachEvent("onclick", existing_tab);
  } else {
    te.addEventListener("onclick", existing_tab);
  }

}

document.addEventListener('DOMContentLoaded', onLoad);

function notify(msg) {
  let notif = document.getElementById('notif');
  notif.innerHTML = msg;
  notif.className = 'open_notif';
  setTimeout(() => {
    document.getElementById('notif').className = 'close_notif';
  }, 5000);
}

function saveFile(content, filename) {
  const blob = new Blob([content], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');

  link.href = url;
  link.download = filename;
  link.click();

  URL.revokeObjectURL(url);
}

function save() {
  console.debug('save id', window.user_id);

  saveFile(window.user_id, 'id.txt');

  notify('ID saved');
}

function load(value) {
  console.debug('load id', value);

  window.user_id = value;
  if(value == "" || value == null){
    window.user_id = uuid();

    localStorage.setItem('id', window.user_id);
    notify('No ID set | Generating new ID');
  }else{
    localStorage.setItem('id', value);
    notify('ID loaded');
  }
}
