using UnityEngine;
using System.Collections;
using UnityEngine.UI;

public class ReadyDetection : MonoBehaviour
{
    public Texture readyTexture;
    public Texture waitTexture;

    public bool leftward;
    RawImage img;
	// Use this for initialization
	void Start ()
    {
        img = gameObject.GetComponent<RawImage>();
	}
	
	// Update is called once per frame
	void Update ()
    {
        if (leftward)
        {
            if (WaveSpawn.LEFT_READY) img.texture = readyTexture;
            else img.texture = waitTexture;
        }
        else
        {
            if (WaveSpawn.RIGHT_READY) img.texture = readyTexture;
            else img.texture = waitTexture;
        }
	}
}
