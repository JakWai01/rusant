struct SaltpaneloOnRequestCallResponse {
  char Accept;
  char *Err;
};

typedef void (*on_request_call)(char *src_id, char *src_email, char *route_id,
                                char *channel_id,
                                struct SaltpaneloOnRequestCallResponse *rv);

typedef void (*on_call_disconnected)(char *route_id, char **rv);

typedef void (*on_handle_call)(char *route_id, char *raddr, char **rv);

typedef void (*open_url)(char *url, char **rv);

void bridge_on_request_call(on_request_call f, char *src_id, char *src_email,
                            char *route_id, char *channel_id,
                            struct SaltpaneloOnRequestCallResponse *rv);

void bridge_on_call_disconnected(on_call_disconnected f, char *route_id,
                                 char **rv);

void bridge_on_handle_call(on_handle_call f, char *route_id, char *raddr,
                           char **rv);

void bridge_open_url(open_url f, char *url, char **rv);
