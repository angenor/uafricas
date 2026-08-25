<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Mes supports médias' }]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes supports médias</h1>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">Les chaînes et stations que vous détenez : leur grille de programmation, les demandes reçues et les personnes qui vous accompagnent.</p>
      </header>
      <MediaMesSupports />
    </div>

    <template #rail>
      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>
