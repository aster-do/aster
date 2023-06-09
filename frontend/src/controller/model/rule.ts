import {
  ApiResponse,
  runDeleteRequest,
  runGetRequest,
  runPostRequest,
} from '../../common/api';
import createUrl from '../../common/api/urlUtils';
import CONTROLLER_API_URL from './api';

export enum RuleOperation {
  ADD = 'Add',
  SUBTRACT = 'Subtract',
  MULTIPLY = 'Multiply',
  DIVIDE = 'Divide',
}

export interface Rule {
  id: number;
  name: string;
  operation: RuleOperation;
  number: number;
  version: number;
}

export interface RuleCreate {
  name: string;
  operation: RuleOperation;
  number: number;
}

export const defaultRule: Rule = {
  id: 0,
  name: '',
  operation: RuleOperation.ADD,
  number: 0,
  version: 0,
};

export function getRuleOperationDisplay(operation: RuleOperation): string {
  switch (operation) {
    case RuleOperation.ADD:
      return '+';
    case RuleOperation.SUBTRACT:
      return '-';
    case RuleOperation.MULTIPLY:
      return '*';
    case RuleOperation.DIVIDE:
      return '/';
    default:
      return '';
  }
}

export async function getRules(): Promise<ApiResponse<Rule[]>> {
  const url = createUrl(`${CONTROLLER_API_URL}/rules`);
  return runGetRequest<Rule[]>(url);
}

export async function getRule(ruleId: number): Promise<ApiResponse<Rule>> {
  return runGetRequest<Rule>(`${CONTROLLER_API_URL}/rules/${ruleId}`);
}

export async function createRule(rule: Rule): Promise<ApiResponse<Rule>> {
  return runPostRequest<Rule, RuleCreate>(`${CONTROLLER_API_URL}/rules`, {
    name: rule.name,
    operation: rule.operation,
    number: rule.number,
  } as RuleCreate);
}

export async function deleteRule(
  ruleId: number
): Promise<ApiResponse<undefined>> {
  return runDeleteRequest<undefined, number>(
    `${CONTROLLER_API_URL}/rules/${ruleId}`
  );
}
