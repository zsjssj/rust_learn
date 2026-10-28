// machine.ts
export type State = 'idle' | 'validating' | 'submitting' | 'success' | 'failed';
export type Event = 'SUBMIT' | 'VALID' | 'INVALID' | 'SUCCESS' | 'FAIL' | 'RETRY' | 'RESET';

interface StateConfig {
  tip?: string;
  disabled?: boolean;
  on?: Partial<Record<Event, State>>; // 该状态能接受的事件
}

export const machine: Record<State, StateConfig> = {
  idle: {
    disabled: false,
    on: { SUBMIT: 'validating' },
  },
  validating: {
    tip: '正在校验…',
    disabled: true,
    on: { VALID: 'submitting', INVALID: 'idle' },
  },
  submitting: {
    tip: '提交中，请稍候…',
    disabled: true,
    on: { SUCCESS: 'success', FAIL: 'failed' },
  },
  success: {
    tip: '提交成功',
    disabled: true,
    on: { RESET: 'idle' },
  },
  failed: {
    tip: '提交失败，可重试',
    disabled: false,
    on: { RETRY: 'submitting', RESET: 'idle' },
  },
};