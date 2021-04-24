using UnityEngine;
using System.Collections;
using UnityEngine.UI;

public class TurnCount : MonoBehaviour {

    public Plane clockable;

    Text count;

	void Start ()
    {
        count = transform.GetChild(0).GetComponent<Text>();
    }
	
	// Update is called once per frame
	void Update ()
    {
        count.text = count.text.Substring(0, count.text.Length - 1);
        count.text += (clockable.clockRate - clockable.getClock());
	}
}
