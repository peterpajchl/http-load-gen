### Manual verification
E.g. we could add query parameters `client_id` and `request_num` which would represent the client's memory address (should be unique enough) and the request counter for set client. Reducing the speed (sleep) on the server side should then help confirm that no more then specific number of clients are probing the service as well as confirming that one client does not affect the execution of the others (they are infact concurrent).

### Is is production ready
Certainly the performance is not suitable for production use. Comparing to the capability of wrk it is significantly slower. The current implementation has at least one significant shortcoming of not having a "load balancer"/queue that would distribute the requests to available connections. Currently in one connection chokes, all requests "assigned" to it will be held up.

### Performence stats

|          | Requests | Req/s  | Connections | Threads |
|----------|----------|--------|-------------|---------|
| wrk      | ~50,000  | 36,226 | 4           | 4       |
| load-gen | 50,000   | 25,947 | 4           | ?       |