using System;
using UnityEngine;

public class SpringJoint2DProxy : MonoBehaviour
{
    // 对应 Spring Joint 2D 面板参数
    [Header("Spring Joint 2D Settings")]
    public Rigidbody2D connectedRigidbody; // 连接的刚体，对应 Connected Rigid Body
    public Vector2 anchor = Vector2.zero;   // 自身锚点，对应 Anchor
    public Vector2 connectedAnchor;         // 连接刚体的锚点，对应 Connected Anchor
    public float distance = 0.3f;           // 弹簧距离，对应 Distance（原 restLength 语义调整）
    public float dampingRatio = 0f;         // 阻尼比，对应 Damping Ratio
    public float frequency = 2f;            // 频率，对应 Frequency
    public BreakAction breakAction = BreakAction.Destroy; // 断连行为，对应 Break Action
    public float breakForce = Mathf.Infinity; // 断裂力，对应 Break Force

    private Rigidbody2D selfRb;
    // 缓存连接刚体的初始速度（可选，看物理需求）
    private Vector2 connectedInitialVelocity; 

    void Start()
    {
        selfRb = GetComponent<Rigidbody2D>();
        if (connectedRigidbody != null)
        {
            connectedInitialVelocity = connectedRigidbody.velocity;
        }
    }

    void FixedUpdate()
    {
        // 先判断是否断裂（简单示例：距离过大超过断裂力逻辑，实际可更复杂）
        float currentDistance = Vector2.Distance(
            selfRb.position + anchor, 
            (connectedRigidbody ? connectedRigidbody.position : Vector2.zero) + connectedAnchor
        );
        if (currentDistance > breakForce && breakAction != BreakAction.DoNothing)
        {
            HandleBreakAction();
            return; 
        }

        // 计算弹簧力（用 Unity 物理逻辑演示，若要 Rust ，把这部分逻辑放 DLL 里调用）
        Vector2 force = CalculateSpringForce();
        selfRb.AddForce(force);

        // 如果有连接刚体，也可以给它反作用力（看需求，真实关节是相互的）
        if (connectedRigidbody != null)
        {
            connectedRigidbody.AddForce(-force);
        }
    }

    // 计算弹簧力，基于 Spring Joint 2D 物理逻辑（简化版）
    private Vector2 CalculateSpringForce()
    {
        // 1. 计算两个锚点的世界位置
        Vector2 selfAnchorWorld = selfRb.position + anchor;
        Vector2 connectedAnchorWorld = connectedRigidbody ? 
            connectedRigidbody.position + connectedAnchor : Vector2.zero;

        // 2. 计算当前长度、相对位移
        float currentLength = Vector2.Distance(selfAnchorWorld, connectedAnchorWorld);
        Vector2 displacement = selfAnchorWorld - connectedAnchorWorld;
        displacement = displacement.normalized * (currentLength - distance); // 形变量

        // 3. 计算相对速度（自身锚点速度 - 连接锚点速度）
        Vector2 selfAnchorVelocity = selfRb.velocity;
        Vector2 connectedAnchorVelocity = connectedRigidbody ? 
            connectedRigidbody.velocity : Vector2.zero;
        Vector2 relativeVelocity = selfAnchorVelocity - connectedAnchorVelocity;

        // 4. 用频率、阻尼比换算刚度和阻尼（参考 Unity Spring Joint 2D 物理公式）
        // 公式简化：k = (2πf)^2 * m ，这里 m 简单取 1 演示，实际可用刚体质量
        float stiffness = Mathf.Pow(2 * Mathf.PI * frequency, 2); 
        float damping = 2 * Mathf.Sqrt(stiffness) * dampingRatio;

        // 5. 胡克定律计算力：F = -k * 形变量  - 阻尼 * 相对速度
        Vector2 springForce = -stiffness * displacement - damping * relativeVelocity;

        return springForce;
    }

    // 处理断连行为
    private void HandleBreakAction()
    {
        switch (breakAction)
        {
            case BreakAction.Destroy:
                Destroy(gameObject); // 销毁自身
                if (connectedRigidbody != null)
                {
                    Destroy(connectedRigidbody.gameObject); // 可选：也销毁连接物体
                }
                break;
            case BreakAction.Disable:
                enabled = false; // 关闭脚本，停止弹簧逻辑
                break;
            case BreakAction.DoNothing:
                // 啥也不做，可用于自定义后续逻辑
                break;
        }
    }

    // 断连行为枚举，对应 Spring Joint 2D 的 Break Action
    public enum BreakAction
    {
        Destroy,
        Disable,
        DoNothing
    }
}

