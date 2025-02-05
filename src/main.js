// 等待 Tauri 准备就绪
document.addEventListener('DOMContentLoaded', () => {
  console.log('DOMContentLoaded');
  
  // 使用 ready 事件
  window.__TAURI__.event.emit('ready').then(async () => {
    console.log('Tauri is ready');
    try {
      // 获取主窗口的位置和大小
      const position = await window.__TAURI__.window.appWindow.innerPosition();
      const size = await window.__TAURI__.window.appWindow.innerSize();

      console.log('Window position and size:', position, size);

      // 创建 Google WebView
      console.log('Creating Google WebView...');
      await window.__TAURI__.tauri.invoke('create_webview', {
        config: {
          url: "https://www.google.com/",
          title: "Google",
          width: 800,
          height: 768,
          x: position.x + size.width + 10,
          y: position.y,
          decorations: true,
          always_on_top: false,
          resizable: true,
          label: "google-view"
        }
      });
      console.log('Google WebView created');

      // 创建 Bing WebView
      console.log('Creating Bing WebView...');
      await window.__TAURI__.tauri.invoke('create_webview', {
        config: {
          url: "https://www.bing.com/",
          title: "Bing",
          width: 800,
          height: 768,
          x: position.x + size.width + 820,
          y: position.y,
          decorations: true,
          always_on_top: false,
          resizable: true,
          label: "bing-view"
        }
      });
      console.log('Bing WebView created');
    } catch (error) {
      console.error('Failed to create webviews:', error);
    }
  }).catch(err => {
    console.error('Failed to emit ready event:', err);
  });
});
