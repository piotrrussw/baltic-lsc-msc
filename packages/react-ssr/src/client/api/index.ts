import axios from "axios";

const HOST = "https://balticlsc.iem.pw.edu.pl/backend";
const PROXY_URL = "http://localhost:3000/";

export enum RequestMethod {
  GET = "get",
  POST = "post",
  PUT = "put",
  PATCH = "patch",
  DELETE = "delete",
}

const API = axios.create();

export const sendRequest = (method: RequestMethod, url: string, data?: any) =>
  API({
    method,
    url: PROXY_URL,
    data,
    headers: {
      "Target-URL": `${HOST}${url}`,
      "Content-Type": "application/json",
    },
  }).then((response) => response.data.data || {});

export const GET_APP_LIST_QUERY_KEY = "/app/list";

export const getAppList = () => {
  const payload = {
    onlyApps: true,
    onlyLastRelease: false,
  };

  return sendRequest(RequestMethod.POST, GET_APP_LIST_QUERY_KEY, payload);
};

export const GET_SHELF_QUERY_KEY = "/app/shelf";

export const getShelf = () => sendRequest(RequestMethod.GET, GET_SHELF_QUERY_KEY);

export const GET_DATA_SHELF_QUERY_KEY = "/task/dataShelf";

export const getDataShelf = () => sendRequest(RequestMethod.GET, GET_DATA_SHELF_QUERY_KEY);

export const getApp = (path: string) => () => sendRequest(RequestMethod.GET, `/app${path}`)