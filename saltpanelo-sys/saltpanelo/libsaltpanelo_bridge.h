struct SaltpaneloOnRequestCallResponse {
  char Accept;
  char *Err;
};

typedef struct SaltpaneloOnRequestCallResponse (*on_request_call_callback)(
    char *src_id, char *src_email, char *route_id, char *channel_id,
    void *userdata);

typedef char *(*on_call_disconnected_callback)(char *route_id, char *channel_id,
                                               void *userdata);

typedef char *(*on_handle_call_callback)(char *route_id, char *channel_id,
                                         char *raddr, void *userdata);

typedef char *(*open_url_callback)(char *url, void *userdata);

struct SaltpaneloOnRequestCallResponse
bridge_on_request_call(on_request_call_callback f, char *src_id,
                       char *src_email, char *route_id, char *channel_id,
                       void *userdata);

char *bridge_on_call_disconnected(on_call_disconnected_callback f,
                                  char *route_id, char *channel_id,
                                  void *userdata);

char *bridge_on_handle_call(on_handle_call_callback f, char *route_id,
                            char *channel_id, char *raddr, void *userdata);

char *bridge_open_url(open_url_callback f, char *url, void *userdata);
