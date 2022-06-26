import copy from "copy-to-clipboard";

const KEY = "p";

export const getQuery = (): string | null => {
  const params = new URLSearchParams(window.location.search);
  const s = params.get(KEY);
  return s ? decodeURIComponent(s) : null;
};

export const copyQuery = (source: string) => {
  const encoded = encodeURIComponent(source);
  const url = new URL(window.location.href);
  if (source.length > 0) {
    url.searchParams.set(KEY, encoded);
  } else {
    url.searchParams.delete(KEY);
  }
  copy(url.toString());
};
