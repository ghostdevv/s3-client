import toTree, { type TreeNode as BaseTreeNode } from 'path-list-to-tree';

export interface TreeNode extends BaseTreeNode {
    path: string;
    children: TreeNode[];
}

function patchNode(node: BaseTreeNode, parentPathSegments: string[]): TreeNode {
    const pathSegments = [...parentPathSegments, node.name];

    return {
        path: pathSegments.join('/'),
        name: node.name,
        children: node.children.map((child) => patchNode(child, pathSegments)),
    };
}

export const tree = (paths: string[]) => {
    const tree = toTree(paths);
    return tree.map((node) => patchNode(node, []));
};
