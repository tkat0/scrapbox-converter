import copy from "copy-to-clipboard";

const KEY_FORM = "p";
const KEY_TAB = "t";

interface GetQueryOutput {
  form: string | null;
  tabIndex: number | null;
}

export const getQuery = (): GetQueryOutput => {
  const params = new URLSearchParams(window.location.search);
  const form = params.get(KEY_FORM);
  const tabIndex = params.get(KEY_TAB);
  return {
    form: form ? decodeURIComponent(form) : null,
    tabIndex: tabIndex ? parseInt(tabIndex) : null,
  };
};

export const copyQuery = (source: string, tabIndex: number) => {
  const encoded = encodeURIComponent(source);
  const url = new URL(window.location.href);
  if (source.length > 0) {
    url.searchParams.set(KEY_FORM, encoded);
  } else {
    url.searchParams.delete(KEY_FORM);
  }
  if (tabIndex != 0) {
    url.searchParams.set(KEY_TAB, tabIndex.toString());
  }
  copy(url.toString());
};
