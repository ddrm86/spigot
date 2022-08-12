import { PiGenerator } from "spigot";

const paragraph = document.getElementById("pi-digit");
const pi_generator = PiGenerator.new();

const restore_size = () => {
    paragraph.style.fontSize = "50px";
    setTimeout(show_next_digit, 1000);
}

const show_next_digit = () => {
    paragraph.textContent = pi_generator.next_digit().toString();
    paragraph.style.fontSize = "400px";
    setTimeout(restore_size, 1000);
};

show_next_digit();
