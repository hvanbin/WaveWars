using UnityEngine;
using System.Collections;

public abstract class Wave : MonoBehaviour
{
    public bool getLeftward() { return leftward; }
    public void setLeftward(bool leftward) { this.leftward = leftward; }

    public virtual void OnTriggerEnter(Collider other)
    {
        if (other.gameObject.tag == "P1")
        {
            if (Plane.WINNER == 1) Plane.WINNER = 3;
            else Plane.WINNER = 2;
        }
        else if (other.gameObject.tag == "P2")
        {
            if (Plane.WINNER == 2) Plane.WINNER = 3;
            else Plane.WINNER = 1;
        }
        if (other.gameObject.tag != "Plane")
        {
			GameObject Splode = Instantiate (Resources.Load ("Waves/'SPLOSION")) as GameObject;
			Splode.transform.position = transform.position;
			if (leftward) {
				Splode.GetComponent<ParticleSystem> ().startColor = Resources.Load<Material> ("Materials/Brown").color;
			} else {
				Splode.GetComponent<ParticleSystem> ().startColor = Resources.Load<Material> ("Materials/Gray").color;
			}
            Destroy(other.gameObject);
            Destroy(gameObject);
        }
    }

    private bool leftward;
    private Vector3 targetPos;

	protected void shiftX(int amount)
    {
		targetPos = new Vector3(transform.position.x + (getLeftward() ? -amount*2 : amount*2), targetPos.y, targetPos.z);
        //if (transform.position.x < -Plane.WIDTH * 3 || transform.position.x > Plane.WIDTH * 3) Destroy(gameObject);
	}
	protected void shiftX()
	{
		shiftX(1);
	}
    protected void shiftY(bool up)
    {
        targetPos = new Vector3(targetPos.x, targetPos.y, transform.position.z + (up ? 2 : -2));
    }
    public abstract void Move();

    public virtual void Start()
    {
        targetPos = new Vector3(transform.position.x, transform.position.y, transform.position.z);
    }
    public virtual void Update()
    {
        if(Vector3.Distance(transform.position, targetPos) > 0.001f)
        {
            transform.position = Vector3.Lerp(transform.position, targetPos, Time.deltaTime * 3);
        }
    }
}