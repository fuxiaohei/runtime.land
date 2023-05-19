/**
 * @fileoverview gRPC-Web generated client stub for moni
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.4.1
// 	protoc              v3.21.12
// source: proto/moni-rpc.proto


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.moni = require('./moni-rpc_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.moni.MoniRpcServiceClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'binary';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.moni.MoniRpcServicePromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'binary';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.LoginEmailRequest,
 *   !proto.moni.LoginResponse>}
 */
const methodDescriptor_MoniRpcService_LoginEmail = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/LoginEmail',
  grpc.web.MethodType.UNARY,
  proto.moni.LoginEmailRequest,
  proto.moni.LoginResponse,
  /**
   * @param {!proto.moni.LoginEmailRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.LoginResponse.deserializeBinary
);


/**
 * @param {!proto.moni.LoginEmailRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.LoginResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.LoginResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.loginEmail =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/LoginEmail',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_LoginEmail,
      callback);
};


/**
 * @param {!proto.moni.LoginEmailRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.LoginResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.loginEmail =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/LoginEmail',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_LoginEmail);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.LoginAccessTokenRequest,
 *   !proto.moni.LoginResponse>}
 */
const methodDescriptor_MoniRpcService_LoginAccessToken = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/LoginAccessToken',
  grpc.web.MethodType.UNARY,
  proto.moni.LoginAccessTokenRequest,
  proto.moni.LoginResponse,
  /**
   * @param {!proto.moni.LoginAccessTokenRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.LoginResponse.deserializeBinary
);


/**
 * @param {!proto.moni.LoginAccessTokenRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.LoginResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.LoginResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.loginAccessToken =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/LoginAccessToken',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_LoginAccessToken,
      callback);
};


/**
 * @param {!proto.moni.LoginAccessTokenRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.LoginResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.loginAccessToken =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/LoginAccessToken',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_LoginAccessToken);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.CreateAccessTokenRequest,
 *   !proto.moni.CreateAccessTokenResponse>}
 */
const methodDescriptor_MoniRpcService_CreateAccessToken = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/CreateAccessToken',
  grpc.web.MethodType.UNARY,
  proto.moni.CreateAccessTokenRequest,
  proto.moni.CreateAccessTokenResponse,
  /**
   * @param {!proto.moni.CreateAccessTokenRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.CreateAccessTokenResponse.deserializeBinary
);


/**
 * @param {!proto.moni.CreateAccessTokenRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.CreateAccessTokenResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.CreateAccessTokenResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.createAccessToken =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/CreateAccessToken',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_CreateAccessToken,
      callback);
};


/**
 * @param {!proto.moni.CreateAccessTokenRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.CreateAccessTokenResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.createAccessToken =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/CreateAccessToken',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_CreateAccessToken);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.Empty,
 *   !proto.moni.ListAccessTokensResponse>}
 */
const methodDescriptor_MoniRpcService_ListAccessTokens = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/ListAccessTokens',
  grpc.web.MethodType.UNARY,
  proto.moni.Empty,
  proto.moni.ListAccessTokensResponse,
  /**
   * @param {!proto.moni.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.ListAccessTokensResponse.deserializeBinary
);


/**
 * @param {!proto.moni.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.ListAccessTokensResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.ListAccessTokensResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.listAccessTokens =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/ListAccessTokens',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_ListAccessTokens,
      callback);
};


/**
 * @param {!proto.moni.Empty} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.ListAccessTokensResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.listAccessTokens =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/ListAccessTokens',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_ListAccessTokens);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.RemoveAccessTokenRequest,
 *   !proto.moni.NoDataResponse>}
 */
const methodDescriptor_MoniRpcService_RemoveAccessToken = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/RemoveAccessToken',
  grpc.web.MethodType.UNARY,
  proto.moni.RemoveAccessTokenRequest,
  proto.moni.NoDataResponse,
  /**
   * @param {!proto.moni.RemoveAccessTokenRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.NoDataResponse.deserializeBinary
);


/**
 * @param {!proto.moni.RemoveAccessTokenRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.NoDataResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.NoDataResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.removeAccessToken =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/RemoveAccessToken',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_RemoveAccessToken,
      callback);
};


/**
 * @param {!proto.moni.RemoveAccessTokenRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.NoDataResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.removeAccessToken =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/RemoveAccessToken',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_RemoveAccessToken);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.CreateProjectRequest,
 *   !proto.moni.CreateProjectResponse>}
 */
const methodDescriptor_MoniRpcService_CreateProject = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/CreateProject',
  grpc.web.MethodType.UNARY,
  proto.moni.CreateProjectRequest,
  proto.moni.CreateProjectResponse,
  /**
   * @param {!proto.moni.CreateProjectRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.CreateProjectResponse.deserializeBinary
);


/**
 * @param {!proto.moni.CreateProjectRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.CreateProjectResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.CreateProjectResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.createProject =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/CreateProject',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_CreateProject,
      callback);
};


/**
 * @param {!proto.moni.CreateProjectRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.CreateProjectResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.createProject =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/CreateProject',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_CreateProject);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.FetchProjectRequest,
 *   !proto.moni.ProjectResponse>}
 */
const methodDescriptor_MoniRpcService_FetchProject = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/FetchProject',
  grpc.web.MethodType.UNARY,
  proto.moni.FetchProjectRequest,
  proto.moni.ProjectResponse,
  /**
   * @param {!proto.moni.FetchProjectRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.ProjectResponse.deserializeBinary
);


/**
 * @param {!proto.moni.FetchProjectRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.ProjectResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.ProjectResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.fetchProject =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/FetchProject',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_FetchProject,
      callback);
};


/**
 * @param {!proto.moni.FetchProjectRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.ProjectResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.fetchProject =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/FetchProject',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_FetchProject);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.FetchProjectRequest,
 *   !proto.moni.ProjectResponse>}
 */
const methodDescriptor_MoniRpcService_CreateEmptyProject = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/CreateEmptyProject',
  grpc.web.MethodType.UNARY,
  proto.moni.FetchProjectRequest,
  proto.moni.ProjectResponse,
  /**
   * @param {!proto.moni.FetchProjectRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.ProjectResponse.deserializeBinary
);


/**
 * @param {!proto.moni.FetchProjectRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.ProjectResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.ProjectResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.createEmptyProject =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/CreateEmptyProject',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_CreateEmptyProject,
      callback);
};


/**
 * @param {!proto.moni.FetchProjectRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.ProjectResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.createEmptyProject =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/CreateEmptyProject',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_CreateEmptyProject);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.moni.Empty,
 *   !proto.moni.ListProjectsResponse>}
 */
const methodDescriptor_MoniRpcService_ListProjects = new grpc.web.MethodDescriptor(
  '/moni.MoniRpcService/ListProjects',
  grpc.web.MethodType.UNARY,
  proto.moni.Empty,
  proto.moni.ListProjectsResponse,
  /**
   * @param {!proto.moni.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.moni.ListProjectsResponse.deserializeBinary
);


/**
 * @param {!proto.moni.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.moni.ListProjectsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.moni.ListProjectsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.moni.MoniRpcServiceClient.prototype.listProjects =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/moni.MoniRpcService/ListProjects',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_ListProjects,
      callback);
};


/**
 * @param {!proto.moni.Empty} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.moni.ListProjectsResponse>}
 *     Promise that resolves to the response
 */
proto.moni.MoniRpcServicePromiseClient.prototype.listProjects =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/moni.MoniRpcService/ListProjects',
      request,
      metadata || {},
      methodDescriptor_MoniRpcService_ListProjects);
};


module.exports = proto.moni;

