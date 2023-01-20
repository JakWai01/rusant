#include "adapter.h"
#include <stdlib.h>

void bridge_on_request_call(on_request_call f, char *src_id, char *src_email,
                            char *route_id, char *channel_id,
                            struct SaltpaneloOnRequestCallResponse *rv) {
  f(src_id, src_email, route_id, channel_id, rv);
}

void bridge_on_call_disconnected(on_call_disconnected f, char *route_id,
                                 char **rv) {
  f(route_id, rv);
}

void bridge_on_handle_call(on_handle_call f, char *route_id, char *raddr,
                           char **rv) {
  f(route_id, raddr, rv);
}

void bridge_open_url(open_url f, char *url, char **rv) { f(url, rv); }