#include "libsaltpanelo_bridge.h"
#include <stdlib.h>

struct SaltpaneloOnRequestCallResponse
bridge_on_request_call(on_request_call_callback f, char *src_id,
                       char *src_email, char *route_id, char *channel_id,
                       void *userdata) {
  return f(src_id, src_email, route_id, channel_id, userdata);
}

char *bridge_on_call_disconnected(on_call_disconnected_callback f,
                                  char *route_id, void *userdata) {
  return f(route_id, userdata);
}

char *bridge_on_handle_call(on_handle_call_callback f, char *route_id,
                            char *raddr, void *userdata) {
  return f(route_id, raddr, userdata);
}

char *bridge_open_url(open_url_callback f, char *url, void *userdata) {
  return f(url, userdata);
}