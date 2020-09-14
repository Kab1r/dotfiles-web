const PRECACHE = "dotfile-kabcache-v2";
const RUNTIME = "runtime";
const CACHED_URLS = [
    '/',
    '/1.dotfiles.js',
    '/dotfiles.js',
    '/dotfiles.wasm',
    "manifest.json",
    "favicon.svg",
    "/browser-specific/android-chrome-192x192.png",
    "/browser-specific/android-chrome-512x512.png",
    "/browser-specific/apple-touch-icon-precomposed.png",
    "/browser-specific/apple-touch-icon.png",
    "/browser-specific/browserconfig.xml",
    "/browser-specific/favicon-16x16.png",
    "/browser-specific/favicon-32x32.png",
    "/browser-specific/mstile-70x70.png",
    "/browser-specific/mstile-144x144.png",
    "/browser-specific/mstile-150x150.png",
    "/browser-specific/mstile-310x150.png",
    "/browser-specific/mstile-310x310.png",
    "/browser-specific/safari-pinned-tab.svg"
];
self.addEventListener('install', event => {
    console.log('Attempting to install service worker and cache static assets');
    event.waitUntil(
        caches.open(PRECACHE)
        .then(cache => {
            return cache.addAll(CACHED_URLS);
        })
    );
});

self.addEventListener('fetch', event => {
    console.log('Fetch event for ', event.request.url);
    event.respondWith(
        caches.match(event.request)
        .then(response => {
            if (response) {
                console.log('Found ', event.request.url, ' in cache');
                return response;
            }
            console.log('Network request for ', event.request.url);
            return fetch(event.request)

            .then(response => {
                // TODO: Respond with custom 404 page
                return caches.open(PRECACHE).then(cache => {
                    cache.put(event.request.url, response.clone());
                    return response;
                });
            });

        }).catch(_ => {})
    );
});