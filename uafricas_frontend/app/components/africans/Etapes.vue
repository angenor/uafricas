<template>
  <!-- Racine UNIQUE : les six appelants passent une classe d'espacement
       (`mb-6`, `my-6`). Un composant à plusieurs racines ne peut pas en
       hériter, et Vue la jetterait avec un simple avertissement. -->
  <div>
    <!-- Fil des étapes d'un formulaire long. Il n'est PAS cliquable en avant :
         on ne saute pas par-dessus une étape dont les champs obligatoires ne
         sont pas remplis. Le retour en arrière, lui, est libre — relire ce
         qu'on a saisi ne casse rien. -->
    <ol class="flex w-full min-w-0 items-center gap-2">
      <!-- `min-w-0` : un élément flex a `min-width: auto` par défaut et REFUSE
           de se réduire sous la largeur de son contenu. Sans lui, le `truncate`
           du libellé ne s'appliquait jamais et la dernière étape débordait du
           cadre — « L'organisation » sortait de la carte. -->
      <!-- C'est le <li> PORTEUR D'UN CONNECTEUR qui s'étire, jamais la
           pastille : le connecteur absorbe l'espace libre et la rangée reste
           tendue d'un bord à l'autre, sans qu'une étape prenne dix fois la
           place de ses voisines. -->
      <li
        v-for="(etape, i) in etapes"
        :key="etape.titre"
        class="flex min-w-0 items-center gap-2"
        :class="i < etapes.length - 1 ? 'flex-1' : ''"
      >
        <button
          type="button"
          class="flex min-w-0 items-center gap-2 rounded-lg px-2 py-1.5 text-left transition"
          :class="[
            i === courante ? 'bg-af-chocolat/10' : '',
            i < courante ? 'hover:bg-af-fond' : '',
            i > courante ? 'cursor-default' : '',
          ]"
          :aria-current="i === courante ? 'step' : undefined"
          :aria-label="`Étape ${i + 1} : ${etape.titre}`"
          :title="etape.titre"
          :disabled="i > courante"
          @click="i < courante && emit('aller', i)"
        >
          <span
            class="grid size-7 shrink-0 place-items-center rounded-full text-[12px] font-bold transition"
            :class="i < courante
              ? 'bg-af-vert text-white'
              : i === courante
                ? 'bg-af-chocolat text-white'
                : 'bg-af-fond text-af-atone-2'"
          >
            <font-awesome-icon v-if="i < courante" icon="fa-solid fa-check" />
            <template v-else>{{ i + 1 }}</template>
          </span>

          <!-- SEULE l'étape courante montre son libellé.
               Une règle au seuil de fenêtre serait ici un faux ami : le gabarit
               réserve la colonne du rail MÊME quand la page n'en a pas, si bien
               que la colonne principale fait ~690 px à toute largeur au-delà de
               1280 px — elle ne s'élargit jamais. Quatre libellés n'y tiennent
               pas, et rognés à trois lettres ils n'informent personne. Le nom
               des autres étapes reste accessible en `title` et en `aria-label`,
               et le compteur sous le fil dit où l'on en est. -->
          <span
            v-if="i === courante"
            class="min-w-0 truncate text-[13px]/[1.3] font-bold text-af-chocolat"
          >
            {{ etape.titre }}
          </span>
        </button>

        <span
          v-if="i < etapes.length - 1"
          class="h-0.5 min-w-4 flex-1 rounded-full"
          :class="i < courante ? 'bg-af-vert' : 'bg-af-bordure'"
        />
      </li>
    </ol>

    <p class="mt-2 text-[12px]/[1.4] text-af-atone">
      Étape {{ courante + 1 }} sur {{ etapes.length }}
    </p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  etapes: readonly { titre: string }[]
  courante: number
}>()

const emit = defineEmits<{ aller: [index: number] }>()
</script>
