
const showNotification = ({ text, type, timeout }) => {
  const id = Date.now().toString();
  const notification = document.createElement('div');

  const notificationText = document.createElement('span');
  notificationText.innerHTML = text;

  const notificationTime = document.createElement('span');
  notificationTime.innerText = new Date().toLocaleTimeString();
  notificationTime.classList.add('time');

  notification.appendChild(notificationText);
  notification.appendChild(notificationTime);
  notification.classList.add(type);
  notification.classList.add('notification');
  notification.id = id;

  document.getElementById('notifications').appendChild(notification);

  notification.addEventListener('click', () =>
    document.getElementById('notifications').removeChild(notification)
  );

  setTimeout(() => {
    document.getElementById('notifications').removeChild(notification);
  }, timeout || 3000);
};
