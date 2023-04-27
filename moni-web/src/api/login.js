const { LoginEmailRequest } = require("./proto/moni-rpc_pb.js");
const { MoniRpcServiceClient } = require("./proto/moni-rpc_grpc_web_pb.js");

function loginByEmail(email, password) {
  let client = new MoniRpcServiceClient("http://127.0.0.1:38779");
  let request = new LoginEmailRequest();
  request.setEmail(email);
  request.setPassword(password);

  let promise = new Promise((resolve, reject) => {
    client.loginEmail(request, {}, (err, response) => {
      if (err) {
        resolve({ code: 1, error: response_error });
        return;
      }
      if (response.getCode()) {
        resolve({ code: response.getCode(), error: response.getError() });
        return;
      }
      let data = response.getData().toObject();
      resolve({ code: 0, data: data });
    });
  });
  return promise;
}

function getLocalUser() {
  let local_user = localStorage.getItem("moni-web-user") || null;
  if (local_user) {
    local_user = JSON.parse(local_user);
  }
  return local_user;
}

function error_text(e) {
  // generate text descrpition for grpc response code
  switch (e.code) {
    case 2:
      return "unknown server error";
  }
  return e.message;
}

function handle_response(response, response_error, callback) {
  if (response_error) {
    callback({ code: 1, error: response_error });
    return;
  }
  if (response.getCode()) {
    console.log("error", response.getCode());
    callback({ code: response.getCode(), error: response.getError() });
    return;
  }
  callback(response);
}

export { loginByEmail, getLocalUser };