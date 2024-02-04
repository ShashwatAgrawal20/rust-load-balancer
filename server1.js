const http = require('http');

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Hello from Server 1\n');
});

const PORT = 5001;
server.listen(PORT, () => {
    console.log(`Server 1 running on port ${PORT}`);
});
