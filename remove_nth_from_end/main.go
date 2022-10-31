/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
type ListNode struct {
		Val int
		Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
    nodes := []*ListNode{head}
    node := *head
    for node.Next != nil {
        nodes = append(nodes, node.Next)
        node = *(node.Next)
    }

    if len(nodes) == 1 {
        return nil
    }
    if len(nodes) == n {
        fmt.Println("ugh")
        return nodes[1]
    }

    fmt.Println(n)
    fmt.Printf("preceding node should now have a next of undesiredNode.next: %v, %v", nodes[len(nodes) - n - 1], nodes[len(nodes) - n].Next)
    
    nodes[len(nodes) - n - 1].Next = nodes[len(nodes) - n].Next

    fmt.Printf("preceding node should now be different: %v", nodes[len(nodes) - n - 1])

    nodes = append(nodes[:len(nodes) - n], nodes[len(nodes) - n + 1:]...)

    fmt.Printf("nodes[0]: %v\n", nodes[0])
    
    return nodes[0]
}