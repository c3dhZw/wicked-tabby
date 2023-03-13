function str() {
  return ('00000000000000000' + (Math.random() * 0xffffffffffffffff).toString(16)).slice(-16);
}

function uuid() {
  var a = str();
  var b = str();

  return a.slice(0, 8) + '-' + a.slice(8, 12) + '-4' + a.slice(13) + '-a' + b.slice(1, 4) + '-' + b.slice(4);
}

function onLoad() {
  window.user_id = localStorage.getItem('id');

  console.log(window.user_id);

  if (window.user_id === null || window.user_id === '') {
    window.user_id = uuid();

    localStorage.setItem('id', window.user_id);
    notify('Generated new ID');
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