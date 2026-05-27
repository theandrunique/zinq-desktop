type ButtonState = "hidden" | "toBottom" | "toLastRead" | "goBack";

interface NavEntry {
  fromId: string;
  toId: string;
}

interface ScrollMessageOptions {
  behavior?: ScrollBehavior;
  block?: ScrollLogicalPosition;
  highlight?: boolean;
}

export type ScrollManager = ReturnType<typeof createScrollManager>;

export function createScrollManager() {
  let buttonState = $state<ButtonState>("hidden");

  // eslint-disable-next-line svelte/prefer-svelte-reactivity
  const elements = new Map<string, HTMLElement>();
  const navStack: NavEntry[] = [];
  let container: HTMLElement | null = null;
  let lastReadId: string = "";
  let isAtBottom = true;
  let isWindowVisible = $state(true);
  let cleanupWindowListeners: (() => void) | null = null;

  function setContainer(el: HTMLElement) {
    container = el;
    el.addEventListener("scroll", updateButtonState);
    updateButtonState();

    if (!cleanupWindowListeners) {
      const onVisibility = () => {
        isWindowVisible = !document.hidden;
      };
      const onFocus = () => {
        isWindowVisible = true;
      };
      const onBlur = () => {
        isWindowVisible = false;
      };

      document.addEventListener("visibilitychange", onVisibility);
      window.addEventListener("focus", onFocus);
      window.addEventListener("blur", onBlur);

      cleanupWindowListeners = () => {
        document.removeEventListener("visibilitychange", onVisibility);
        window.removeEventListener("focus", onFocus);
        window.removeEventListener("blur", onBlur);
      };
    }
  }

  function messageAction(node: HTMLElement, id: string) {
    elements.set(id, node);
    return {
      destroy() {
        elements.delete(id);
      },
    };
  }

  function updateButtonState() {
    if (!container) {
      buttonState = "hidden";
      return;
    }

    const { scrollHeight, scrollTop, clientHeight } = container;
    isAtBottom = scrollHeight - scrollTop - clientHeight < 100;

    if (navStack.length > 0) {
      buttonState = "goBack";
      return;
    }

    if (isAtBottom) {
      buttonState = "hidden";
      return;
    }

    if (lastReadId && elements.has(lastReadId)) {
      const lastReadEl = elements.get(lastReadId)!;
      const rect = lastReadEl.getBoundingClientRect();
      const containerRect = container.getBoundingClientRect();
      if (rect.top > containerRect.bottom) {
        buttonState = "toLastRead";
        return;
      }
    }

    buttonState = "toBottom";
  }

  function scrollToMessage(id: string, opts?: ScrollMessageOptions) {
    const el = elements.get(id);
    if (!el) return;

    const { behavior = "smooth", block = "nearest", highlight = false } = opts ?? {};

    el.scrollIntoView({ behavior, block });

    if (highlight) {
      el.style.backgroundColor = "color-mix(in srgb, var(--color-accent), transparent 85%)";
      el.style.transition = "background-color 0.4s ease";
      setTimeout(() => {
        el.style.backgroundColor = "";
        el.style.transition = "";
      }, 2000);
    }

    setTimeout(updateButtonState, 350);
  }

  function scrollToBottom(behavior: ScrollBehavior = "smooth") {
    container?.scrollTo({ top: container.scrollHeight, behavior });
    setTimeout(updateButtonState, 350);
  }

  function scrollToLastRead() {
    if (lastReadId && elements.has(lastReadId)) {
      scrollToMessage(lastReadId);
    } else {
      scrollToBottom();
    }
  }

  function scrollToBottomOnNewMessage() {
    if (!container) return;
    const { scrollHeight, scrollTop, clientHeight } = container;
    const atBottom = scrollHeight - scrollTop - clientHeight < 100;

    if (atBottom && isWindowVisible) {
      scrollToBottom();
    } else {
      updateButtonState();
    }
  }

  function navigateToReference(fromId: string, toId: string) {
    navStack.push({ fromId, toId });
    scrollToMessage(toId, { block: "center", highlight: true });
  }

  function goBack() {
    const entry = navStack.pop();
    if (!entry) return;
    scrollToMessage(entry.fromId, { block: "center", highlight: true });
  }

  function setLastReadId(id: string) {
    lastReadId = id;
  }

  function destroy() {
    container?.removeEventListener("scroll", updateButtonState);
    cleanupWindowListeners?.();
    elements.clear();
    navStack.length = 0;
  }

  return {
    get buttonState() {
      return buttonState;
    },
    messageAction,
    setContainer,
    scrollToBottom,
    scrollToMessage,
    scrollToLastRead,
    scrollToBottomOnNewMessage,
    navigateToReference,
    goBack,
    setLastReadId,
    updateButtonState,
    destroy,
  };
}
