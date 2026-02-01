<script setup lang="ts">
defineProps<{
  total: number
}>()
</script>

<template>
  <div
    class="text-2xl font-bold p-5 bg-white rounded-r-xl mt-3 border-l-4 border-custom-green"
    data-aos="fade-up"
    data-aos-duration="800"
  >
    <div class="flex flex-wrap items-center gap-3">
      <div class="text-custom-chocolat">
        Centres culturels africain et afro-descendant
      </div>
      <div class="flex text-base font-normal">
        <div
          class="border rounded-md px-3 py-0.5 text-custom-green border-custom-chocolat"
        >
          Total: {{ total }}
        </div>
      </div>
    </div>
    <div class="text-base font-normal mt-3 text-gray-700">
      Les centres culturels africain et afro-descendant (CCAD) sont une
      innovation d'Africans destinée à mettre en avant les valeurs et les
      bonnes pratiques communes aux peuples issus ou descendant d'Afrique.
      Ces centres sont installés sous la coordination d'Africans en
      considérant l'intérêt et la population cible. Ils peuvent donc être
      installés aussi dans les territoires d'Afrique que dans les pays hors
      d'Afrique. Les centres culturels portent les activités suivantes en
      ligne ou en présentiel :
      <ul class="list-disc ml-10 mt-2">
        <li>Rayonnement de la culture africaine et afro-descendante</li>
        <li>Réseautage intra-territoires, ethniques, etc.</li>
        <li>Valorisation des alliances inter-ethniques</li>
        <li>Promotion d'artistes mettant en avant les traditions cibles</li>
        <li>
          Renforcement des capacités des enfants ou des adultes sur les
          langues africaines ou afro-descendantes
        </li>
        <li>
          Promotion des bonnes pratiques d'origine africaines sur les enjeux
          globaux (environnement, gouvernance, éducation, etc.).
        </li>
      </ul>
    </div>
  </div>
</template>
