/// Each variant descibes a specific AST node type
enum ASTType
{
    ROOT, // Translation unit root
    FUNCDECL(ASTFuncDecl),
    PARAMDECL,
    COMPOUNDSTMT,
    RETURNSTMT,
    DECLREFEXPR,
    BINARYOP,
    LITERAL
}

struct ASTFuncDecl {

}

/// Single node of an Abstract Syntax Tree
struct ASTNode<'a> {
    node_type: ASTType,

    parent: Option<&'a ASTNode<'a>>,
    children: Vec<ASTNode<'a>>,
}
impl<'a> ASTNode<'a> {
    fn add_child(self, node: ASTType) {
        let child = ASTNode{};
    }
}

struct AST<'a> {
    root: ASTNode<'a>,
}
impl<'a> AST<'a> {
    fn new() -> AST<'a> {
        AST{root: ASTNode{node_type: ASTType::ROOT,
                          parent: None,
                          children: Vec::new()}
                        }
    }
}
