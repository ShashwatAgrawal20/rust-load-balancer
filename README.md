# A shitty multi-threaded load balancer

This is using the round-robin algorithm, might implement the least connection in future.

**Note:- The LB doesn't check for the server status wheather they are up or down**
- I didn't wanted to implement the quick workaround I had in my mind which was -> call the `process_request` function again if there's a error connecting to the current server.

- I might implement a better approach to this like dynamically updating the server list according to the status.

we can configure `./server_list.txt` to list the servers we want the load balancer to work on.

> format -> `IP:PORT`


### Start the Load Balancer
``` bash
./load_balancer.sh
```

### Start Testing Servers
Just for testing purposes I have created a shitty script that will spawn 3 http servers written in nodejs.

```bash
node ./testing_servers.js
```

### Testing the Load Blancer

just navigate to http://localhost:6969 in your browser and try refreshing the page.

OR

```bash
curl localhost:6969
```
